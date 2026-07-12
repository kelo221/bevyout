[bevy](../../index.html)::[math](../index.html)::[prelude](index.html)

# Struct UVec2 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#31)

```rust
#[repr(C)]pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}
```

A 2-dimensional vector.

## Fields

`x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)``y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#36)

### impl [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#38)

#### pub const [ZERO](#associatedconstant.ZERO): [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

All zeroes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#41)

#### pub const [ONE](#associatedconstant.ONE): [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

All ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#44)

#### pub const [MIN](#associatedconstant.MIN): [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

All `u32::MIN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#47)

#### pub const [MAX](#associatedconstant.MAX): [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

All `u32::MAX`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#50)

#### pub const [X](#associatedconstant.X): [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

A unit vector pointing along the positive X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#53)

#### pub const [Y](#associatedconstant.Y): [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

A unit vector pointing along the positive Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#56)

#### pub const [AXES](#associatedconstant.AXES): \[[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The unit axes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#61)

#### pub const fn [new](#method.new)(x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Creates a new vector.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/shader/compute\_shader\_game\_of\_life.rs ([line 29](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#29))

```rust
29const SIZE: UVec2 = UVec2::new(1280 / DISPLAY_FACTOR, 720 / DISPLAY_FACTOR);
```

Hide additional examples

examples/window/persisting\_window\_settings.rs ([lines 121-124](../../../src/persisting_window_settings/persisting_window_settings.rs.html#121-124))

```rust
115fn store_window_settings(mut window_settings: ResMut<WindowSettings>, window: &Window) -> bool {
116    window_settings.set_if_neq(WindowSettings {
117        position: match window.position {
118            WindowPosition::At(pos) => Some(pos),
119            _ => None,
120        },
121        size: Some(UVec2::new(
122            window.resolution.width() as u32,
123            window.resolution.height() as u32,
124        )),
125        fullscreen: window.mode != WindowMode::Windowed,
126    })
127}
```

examples/window/custom\_cursor\_image.rs ([line 201](../../../src/custom_cursor_image/custom_cursor_image.rs.html#201))

```rust
195    const SECTIONS: [Option<URect>; 5] = [
196        Some(URect {
197            min: UVec2::ZERO,
198            max: UVec2::splat(RECT_SIZE),
199        }),
200        Some(URect {
201            min: UVec2::new(RECT_SIZE, 0),
202            max: UVec2::new(2 * RECT_SIZE, RECT_SIZE),
203        }),
204        Some(URect {
205            min: UVec2::new(0, RECT_SIZE),
206            max: UVec2::new(RECT_SIZE, 2 * RECT_SIZE),
207        }),
208        Some(URect {
209            min: UVec2::new(RECT_SIZE, RECT_SIZE),
210            max: UVec2::splat(2 * RECT_SIZE),
211        }),
212        None, // reset to None
213    ];
```

examples/2d/tilemap\_chunk.rs ([line 76](../../../src/tilemap_chunk/tilemap_chunk.rs.html#76))

```rust
70fn spawn_fake_player(
71    mut commands: Commands,
72    mut meshes: ResMut<Assets<Mesh>>,
73    mut materials: ResMut<Assets<ColorMaterial>>,
74    chunk: Single<&TilemapChunk>,
75) {
76    let mut transform = chunk.calculate_tile_transform(UVec2::new(0, 0));
77    transform.translation.z = 1.;
78
79    commands.spawn((
80        Mesh2d(meshes.add(Rectangle::new(8., 8.))),
81        MeshMaterial2d(materials.add(Color::from(RED_400))),
82        transform,
83        MovePlayer,
84    ));
85
86    let mut transform = chunk.calculate_tile_transform(UVec2::new(5, 6));
87    transform.translation.z = 1.;
88
89    // second "player" to visually test a non-zero position
90    commands.spawn((
91        Mesh2d(meshes.add(Rectangle::new(8., 8.))),
92        MeshMaterial2d(materials.add(Color::from(RED_400))),
93        transform,
94    ));
95}
96
97fn move_player(
98    mut player: Single<&mut Transform, With<MovePlayer>>,
99    time: Res<Time>,
100    chunk: Single<&TilemapChunk>,
101) {
102    let t = (ops::sin(time.elapsed_secs()) + 1.) / 2.;
103
104    let origin = chunk
105        .calculate_tile_transform(UVec2::new(0, 0))
106        .translation
107        .x;
108    let destination = chunk
109        .calculate_tile_transform(UVec2::new(63, 0))
110        .translation
111        .x;
112
113    player.translation.x = origin.lerp(destination, t);
114}
115
116fn update_tilemap(
117    time: Res<Time>,
118    mut query: Query<(&mut TilemapChunkTileData, &mut UpdateTimer)>,
119    mut rng: ResMut<SeededRng>,
120) {
121    for (mut tile_data, mut timer) in query.iter_mut() {
122        timer.tick(time.delta());
123
124        if timer.just_finished() {
125            for _ in 0..50 {
126                let index = rng.random_range(0..tile_data.len());
127                tile_data[index] = Some(TileData::from_tileset_index(rng.random_range(0..5)));
128            }
129        }
130    }
131}
132
133// find the data for an arbitrary tile in the chunk and log its data
134fn log_tile(tilemap: Single<(&TilemapChunk, &TilemapChunkTileData)>, mut local: Local<u16>) {
135    let (chunk, data) = tilemap.into_inner();
136    let Some(tile_data) = data.tile_data_from_tile_pos(chunk.chunk_size, UVec2::new(3, 4)) else {
137        return;
138    };
139    // log when the tile changes
140    if tile_data.tileset_index != *local {
141        info!(?tile_data, "tile_data changed");
142        *local = tile_data.tileset_index;
143    }
144}
```

examples/testbed/2d.rs ([line 422](../../../src/testbed_2d/2d.rs.html#422))

```rust
394    pub fn draw_gizmos(mut gizmos: Gizmos) {
395        gizmos.rect_2d(
396            Isometry2d::from_translation(Vec2::new(-200.0, 0.0)),
397            Vec2::new(200.0, 200.0),
398            RED,
399        );
400        gizmos
401            .circle_2d(
402                Isometry2d::from_translation(Vec2::new(-200.0, 0.0)),
403                200.0,
404                GREEN,
405            )
406            .resolution(64);
407
408        gizmos.text_2d(
409            Isometry2d::from_translation(Vec2::new(-200.0, 0.0)),
410            "text_2d gizmo",
411            15.,
412            Vec2 { x: 0., y: 0. },
413            Color::WHITE,
414        );
415
416        // 2d grids with all variations of outer edges on or off
417        for i in 0..4 {
418            let x = 200.0 * (1.0 + (i % 2) as f32);
419            let y = 150.0 * (0.5 - (i / 2) as f32);
420            let mut grid = gizmos.grid(
421                Vec3::new(x, y, 0.0),
422                UVec2::new(5, 4),
423                Vec2::splat(30.),
424                Color::WHITE,
425            );
426            if i & 1 > 0 {
427                grid = grid.outer_edges_x();
428            }
429            if i & 2 > 0 {
430                grid.outer_edges_y();
431            }
432        }
433    }
```

examples/picking/sprite\_picking.rs ([line 128](../../../src/sprite_picking/sprite_picking.rs.html#128))

```rust
122fn setup_atlas(
123    mut commands: Commands,
124    asset_server: Res<AssetServer>,
125    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
126) {
127    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
128    let layout = TextureAtlasLayout::from_grid(UVec2::new(24, 24), 7, 1, None, None);
129    let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
130    // Use only the subset of sprites in the sheet that make up the run animation
131    let animation_indices = AnimationIndices { first: 1, last: 6 };
132    commands
133        .spawn((
134            Sprite::from_atlas_image(
135                texture_handle,
136                TextureAtlas {
137                    layout: texture_atlas_layout_handle,
138                    index: animation_indices.first,
139                },
140            ),
141            Transform::from_xyz(300.0, 0.0, 0.0).with_scale(Vec3::splat(6.0)),
142            animation_indices,
143            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
144            Pickable::default(),
145        ))
146        .observe(recolor_on::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
147        .observe(recolor_on::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)))
148        .observe(recolor_on::<Pointer<Press>>(Color::srgb(1.0, 1.0, 0.0)))
149        .observe(recolor_on::<Pointer<Release>>(Color::srgb(0.0, 1.0, 1.0)));
150}
```

Additional examples can be found in:  

*   [examples/shader/automatic\_instancing.rs](../../../src/automatic_instancing/automatic_instancing.rs.html#46)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../../src/many_cameras_lights/many_cameras_lights.rs.html#80-83)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#54)
*   [examples/gizmos/2d\_gizmos.rs](../../../src/2d_gizmos/2d_gizmos.rs.html#53)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#122)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#131)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#791)
*   [examples/3d/split\_screen.rs](../../../src/split_screen/split_screen.rs.html#79)
*   [examples/2d/texture\_atlas.rs](../../../src/texture_atlas/texture_atlas.rs.html#81)
*   [examples/3d/camera\_sub\_view.rs](../../../src/camera_sub_view/camera_sub_view.rs.html#80)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#68)

#### pub const fn [splat](#method.splat)(v: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Creates a vector with all elements set to `v`.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/testbed/2d.rs ([line 445](../../../src/testbed_2d/2d.rs.html#445))

```rust
445    const ATLAS_SIZE: UVec2 = UVec2::splat(64);
446    const IMAGE_SIZE: UVec2 = UVec2::splat(28);
447    const PADDING_SIZE: UVec2 = UVec2::splat(2);
```

Hide additional examples

examples/2d/dynamic\_mip\_generation.rs ([line 412](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#412))

```rust
406    fn next(&mut self) -> Option<Self::Item> {
407        // The size of mipmap level N + 1 is equal to half the size of mipmap
408        // level N, rounding down, except that the size can never go below 1
409        // pixel on either axis.
410        let result = self.size;
411        if let Some(size) = self.size {
412            self.size = if size == UVec2::splat(1) {
413                None
414            } else {
415                Some((size / 2).max(UVec2::splat(1)))
416            };
417        }
418        result
419    }
```

examples/2d/sprite\_sheet.rs ([line 48](../../../src/sprite_sheet/sprite_sheet.rs.html#48))

```rust
42fn setup(
43    mut commands: Commands,
44    asset_server: Res<AssetServer>,
45    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
46) {
47    let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
48    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
49    let texture_atlas_layout = texture_atlas_layouts.add(layout);
50    // Use only the subset of sprites in the sheet that make up the run animation
51    let animation_indices = AnimationIndices { first: 1, last: 6 };
52
53    commands.spawn(Camera2d);
54
55    commands.spawn((
56        Sprite::from_atlas_image(
57            texture,
58            TextureAtlas {
59                layout: texture_atlas_layout,
60                index: animation_indices.first,
61            },
62        ),
63        Transform::from_scale(Vec3::splat(6.0)),
64        animation_indices,
65        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
66    ));
67}
```

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 255](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#255))

```rust
240fn spawn_depth_only_camera(commands: &mut Commands) {
241    commands.spawn((
242        Camera3d::default(),
243        Transform::from_xyz(-4.0, -5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
244        Camera {
245            // Make sure that we render from this depth-only camera *before*
246            // rendering from the main camera.
247            order: -1,
248            ..Camera::default()
249        },
250        // We specify no color render target, for maximum efficiency.
251        RenderTarget::None {
252            // When specifying no render target, we must manually specify
253            // the viewport size. Otherwise, Bevy won't know how big to make
254            // the depth buffer.
255            size: UVec2::splat(DEPTH_TEXTURE_SIZE),
256        },
257        // We need to disable multisampling or the depth texture will be
258        // multisampled, which adds complexity we don't care about for this
259        // demo.
260        Msaa::Off,
261        // Cameras with no render target render *nothing* by default. To get
262        // them to render something, we must add a prepass that specifies what
263        // we want to render: in this case, depth.
264        DepthPrepass,
265    ));
266}
```

examples/window/custom\_cursor\_image.rs ([line 38](../../../src/custom_cursor_image/custom_cursor_image.rs.html#38))

```rust
31fn setup_cursor_icon(
32    mut commands: Commands,
33    asset_server: Res<AssetServer>,
34    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
35    window: Single<Entity, With<Window>>,
36) {
37    let layout =
38        TextureAtlasLayout::from_grid(UVec2::splat(64), 20, 10, Some(UVec2::splat(5)), None);
39    let texture_atlas_layout = texture_atlas_layouts.add(layout);
40
41    let animation_config = AnimationConfig::new(0, 199, 1, 4);
42
43    commands.entity(*window).insert((
44        CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
45            // Image to use as the cursor.
46            handle: asset_server
47                .load("cursors/kenney_crosshairPack/Tilesheet/crosshairs_tilesheet_white.png"),
48            // Optional texture atlas allows you to pick a section of the image
49            // and animate it.
50            texture_atlas: Some(TextureAtlas {
51                layout: texture_atlas_layout.clone(),
52                index: animation_config.first_sprite_index,
53            }),
54            flip_x: false,
55            flip_y: false,
56            // Optional section of the image to use as the cursor.
57            rect: None,
58            // The hotspot is the point in the cursor image that will be
59            // positioned at the mouse cursor's position.
60            hotspot: (0, 0),
61        })),
62        animation_config,
63    ));
64}
65
66fn setup_camera(mut commands: Commands) {
67    commands.spawn(Camera3d::default());
68}
69
70fn setup_instructions(mut commands: Commands) {
71    commands.spawn((
72        Text::new(
73            "Press T to toggle the cursor's `texture_atlas`.\n
74Press X to toggle the cursor's `flip_x` setting.\n
75Press Y to toggle the cursor's `flip_y` setting.\n
76Press C to cycle through the sections of the cursor's image using `rect`.",
77        ),
78        Node {
79            position_type: PositionType::Absolute,
80            bottom: px(12),
81            left: px(12),
82            ..default()
83        },
84    ));
85}
86
87#[derive(Component)]
88struct AnimationConfig {
89    first_sprite_index: usize,
90    last_sprite_index: usize,
91    increment: usize,
92    fps: u8,
93    frame_timer: Timer,
94}
95
96impl AnimationConfig {
97    fn new(first: usize, last: usize, increment: usize, fps: u8) -> Self {
98        Self {
99            first_sprite_index: first,
100            last_sprite_index: last,
101            increment,
102            fps,
103            frame_timer: Self::timer_from_fps(fps),
104        }
105    }
106
107    fn timer_from_fps(fps: u8) -> Timer {
108        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
109    }
110}
111
112/// This system loops through all the sprites in the [`CursorIcon`]'s
113/// [`TextureAtlas`], from [`AnimationConfig`]'s `first_sprite_index` to
114/// `last_sprite_index`.
115fn execute_animation(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut CursorIcon)>) {
116    for (mut config, mut cursor_icon) in &mut query {
117        if let CursorIcon::Custom(CustomCursor::Image(ref mut image)) = *cursor_icon {
118            config.frame_timer.tick(time.delta());
119
120            if config.frame_timer.is_finished()
121                && let Some(atlas) = image.texture_atlas.as_mut()
122            {
123                atlas.index += config.increment;
124
125                if atlas.index > config.last_sprite_index {
126                    atlas.index = config.first_sprite_index;
127                }
128
129                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
130            }
131        }
132    }
133}
134
135fn toggle_texture_atlas(
136    input: Res<ButtonInput<KeyCode>>,
137    mut query: Query<&mut CursorIcon, With<Window>>,
138    mut cached_atlas: Local<Option<TextureAtlas>>, // this lets us restore the previous value
139) {
140    if input.just_pressed(KeyCode::KeyT) {
141        for mut cursor_icon in &mut query {
142            if let CursorIcon::Custom(CustomCursor::Image(ref mut image)) = *cursor_icon {
143                match image.texture_atlas.take() {
144                    Some(a) => {
145                        // Save the current texture atlas.
146                        *cached_atlas = Some(a.clone());
147                    }
148                    None => {
149                        // Restore the cached texture atlas.
150                        if let Some(cached_a) = cached_atlas.take() {
151                            image.texture_atlas = Some(cached_a);
152                        }
153                    }
154                }
155            }
156        }
157    }
158}
159
160fn toggle_flip_x(
161    input: Res<ButtonInput<KeyCode>>,
162    mut query: Query<&mut CursorIcon, With<Window>>,
163) {
164    if input.just_pressed(KeyCode::KeyX) {
165        for mut cursor_icon in &mut query {
166            if let CursorIcon::Custom(CustomCursor::Image(ref mut image)) = *cursor_icon {
167                image.flip_x = !image.flip_x;
168            }
169        }
170    }
171}
172
173fn toggle_flip_y(
174    input: Res<ButtonInput<KeyCode>>,
175    mut query: Query<&mut CursorIcon, With<Window>>,
176) {
177    if input.just_pressed(KeyCode::KeyY) {
178        for mut cursor_icon in &mut query {
179            if let CursorIcon::Custom(CustomCursor::Image(ref mut image)) = *cursor_icon {
180                image.flip_y = !image.flip_y;
181            }
182        }
183    }
184}
185
186/// This system alternates the [`CursorIcon`]'s `rect` field between `None` and
187/// 4 sections/rectangles of the cursor's image.
188fn cycle_rect(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut CursorIcon, With<Window>>) {
189    if !input.just_pressed(KeyCode::KeyC) {
190        return;
191    }
192
193    const RECT_SIZE: u32 = 32; // half the size of a tile in the texture atlas
194
195    const SECTIONS: [Option<URect>; 5] = [
196        Some(URect {
197            min: UVec2::ZERO,
198            max: UVec2::splat(RECT_SIZE),
199        }),
200        Some(URect {
201            min: UVec2::new(RECT_SIZE, 0),
202            max: UVec2::new(2 * RECT_SIZE, RECT_SIZE),
203        }),
204        Some(URect {
205            min: UVec2::new(0, RECT_SIZE),
206            max: UVec2::new(RECT_SIZE, 2 * RECT_SIZE),
207        }),
208        Some(URect {
209            min: UVec2::new(RECT_SIZE, RECT_SIZE),
210            max: UVec2::splat(2 * RECT_SIZE),
211        }),
212        None, // reset to None
213    ];
```

examples/2d/tilemap\_chunk.rs ([line 31](../../../src/tilemap_chunk/tilemap_chunk.rs.html#31))

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

*   [examples/usage/cooldown.rs](../../../src/cooldown/cooldown.rs.html#29)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../../src/ui_texture_atlas/ui_texture_atlas.rs.html#30)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../../src/many_animated_sprites/many_animated_sprites.rs.html#63)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#65)
*   [examples/2d/sprite\_animation.rs](../../../src/sprite_animation/sprite_animation.rs.html#107)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#18)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#54)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#106)
*   [examples/2d/sprite\_scale.rs](../../../src/sprite_scale/sprite_scale.rs.html#144)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#75-77)

#### pub fn [map](#method.map)<F>(self, f: F) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html),

Returns a vector containing each element of `self` modified by a mapping function `f`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#89)

#### pub fn [select](#method.select)(mask: [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2"), if\_true: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), if\_false: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Creates a vector from the elements in `if_true` and `if_false`, selecting which to use for each element of `self`.

A true element in the mask uses the corresponding element from `if_true`, and false uses the element from `if_false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#99)

#### pub const fn [from\_array](#method.from_array)(a: \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Creates a new vector from an array.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#106)

#### pub const fn [to\_array](#method.to_array)(&self) -> \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts `self` to `[x, y]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#117)

#### pub const fn [from\_slice](#method.from_slice)(slice: &\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\]) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Creates a vector from the first 2 values in `slice`.

##### Panics

Panics if `slice` is less than 2 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#128)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\])

Writes the elements of `self` to the first 2 elements in `slice`.

##### Panics

Panics if `slice` is less than 2 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#135)

#### pub const fn [extend](#method.extend)(self, z: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

Creates a 3D vector from `self` and the given `z` value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#142)

#### pub fn [with\_x](#method.with_x)(self, x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Creates a 2D vector from `self` with the given value of `x`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#150)

#### pub fn [with\_y](#method.with_y)(self, y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Creates a 2D vector from `self` with the given value of `y`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#158)

#### pub fn [dot](#method.dot)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Computes the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#165)

#### pub fn [dot\_into\_vec](#method.dot_into_vec)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector where every component is the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#174)

#### pub fn [min](#method.min)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the minimum values for each element of `self` and `rhs`.

In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/2d/2d\_viewport\_to\_world.rs ([line 102](../../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#102))

```rust
42fn controls(
43    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
44    window: Single<&Window>,
45    input: Res<ButtonInput<KeyCode>>,
46    time: Res<Time<Fixed>>,
47) {
48    let (mut camera, mut transform, mut projection) = camera_query.into_inner();
49
50    let fspeed = 600.0 * time.delta_secs();
51    let uspeed = fspeed as u32;
52    let window_size = window.resolution.physical_size();
53
54    // Camera movement controls
55    if input.pressed(KeyCode::ArrowUp) {
56        transform.translation.y += fspeed;
57    }
58    if input.pressed(KeyCode::ArrowDown) {
59        transform.translation.y -= fspeed;
60    }
61    if input.pressed(KeyCode::ArrowLeft) {
62        transform.translation.x -= fspeed;
63    }
64    if input.pressed(KeyCode::ArrowRight) {
65        transform.translation.x += fspeed;
66    }
67
68    // Camera zoom controls
69    if let Projection::Orthographic(projection2d) = &mut *projection {
70        if input.pressed(KeyCode::Comma) {
71            projection2d.scale *= powf(4.0f32, time.delta_secs());
72        }
73
74        if input.pressed(KeyCode::Period) {
75            projection2d.scale *= powf(0.25f32, time.delta_secs());
76        }
77    }
78
79    if let Some(viewport) = camera.viewport.as_mut() {
80        // Reset viewport size on window resize
81        if viewport.physical_size.x > window_size.x || viewport.physical_size.y > window_size.y {
82            viewport.physical_size = (window_size.as_vec2() * 0.75).as_uvec2();
83        }
84
85        // Viewport movement controls
86        if input.pressed(KeyCode::KeyW) {
87            viewport.physical_position.y = viewport.physical_position.y.saturating_sub(uspeed);
88        }
89        if input.pressed(KeyCode::KeyS) {
90            viewport.physical_position.y += uspeed;
91        }
92        if input.pressed(KeyCode::KeyA) {
93            viewport.physical_position.x = viewport.physical_position.x.saturating_sub(uspeed);
94        }
95        if input.pressed(KeyCode::KeyD) {
96            viewport.physical_position.x += uspeed;
97        }
98
99        // Bound viewport position so it doesn't go off-screen
100        viewport.physical_position = viewport
101            .physical_position
102            .min(window_size - viewport.physical_size);
103
104        // Viewport size controls
105        if input.pressed(KeyCode::KeyI) {
106            viewport.physical_size.y = viewport.physical_size.y.saturating_sub(uspeed);
107        }
108        if input.pressed(KeyCode::KeyK) {
109            viewport.physical_size.y += uspeed;
110        }
111        if input.pressed(KeyCode::KeyJ) {
112            viewport.physical_size.x = viewport.physical_size.x.saturating_sub(uspeed);
113        }
114        if input.pressed(KeyCode::KeyL) {
115            viewport.physical_size.x += uspeed;
116        }
117
118        // Bound viewport size so it doesn't go off-screen
119        viewport.physical_size = viewport
120            .physical_size
121            .min(window_size - viewport.physical_position)
122            .max(UVec2::new(20, 20));
123    }
124}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#186)

#### pub fn [max](#method.max)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the maximum values for each element of `self` and `rhs`.

In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/2d/dynamic\_mip\_generation.rs ([line 415](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#415))

```rust
406    fn next(&mut self) -> Option<Self::Item> {
407        // The size of mipmap level N + 1 is equal to half the size of mipmap
408        // level N, rounding down, except that the size can never go below 1
409        // pixel on either axis.
410        let result = self.size;
411        if let Some(size) = self.size {
412            self.size = if size == UVec2::splat(1) {
413                None
414            } else {
415                Some((size / 2).max(UVec2::splat(1)))
416            };
417        }
418        result
419    }
```

Hide additional examples

examples/2d/2d\_viewport\_to\_world.rs ([line 122](../../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#122))

```rust
42fn controls(
43    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
44    window: Single<&Window>,
45    input: Res<ButtonInput<KeyCode>>,
46    time: Res<Time<Fixed>>,
47) {
48    let (mut camera, mut transform, mut projection) = camera_query.into_inner();
49
50    let fspeed = 600.0 * time.delta_secs();
51    let uspeed = fspeed as u32;
52    let window_size = window.resolution.physical_size();
53
54    // Camera movement controls
55    if input.pressed(KeyCode::ArrowUp) {
56        transform.translation.y += fspeed;
57    }
58    if input.pressed(KeyCode::ArrowDown) {
59        transform.translation.y -= fspeed;
60    }
61    if input.pressed(KeyCode::ArrowLeft) {
62        transform.translation.x -= fspeed;
63    }
64    if input.pressed(KeyCode::ArrowRight) {
65        transform.translation.x += fspeed;
66    }
67
68    // Camera zoom controls
69    if let Projection::Orthographic(projection2d) = &mut *projection {
70        if input.pressed(KeyCode::Comma) {
71            projection2d.scale *= powf(4.0f32, time.delta_secs());
72        }
73
74        if input.pressed(KeyCode::Period) {
75            projection2d.scale *= powf(0.25f32, time.delta_secs());
76        }
77    }
78
79    if let Some(viewport) = camera.viewport.as_mut() {
80        // Reset viewport size on window resize
81        if viewport.physical_size.x > window_size.x || viewport.physical_size.y > window_size.y {
82            viewport.physical_size = (window_size.as_vec2() * 0.75).as_uvec2();
83        }
84
85        // Viewport movement controls
86        if input.pressed(KeyCode::KeyW) {
87            viewport.physical_position.y = viewport.physical_position.y.saturating_sub(uspeed);
88        }
89        if input.pressed(KeyCode::KeyS) {
90            viewport.physical_position.y += uspeed;
91        }
92        if input.pressed(KeyCode::KeyA) {
93            viewport.physical_position.x = viewport.physical_position.x.saturating_sub(uspeed);
94        }
95        if input.pressed(KeyCode::KeyD) {
96            viewport.physical_position.x += uspeed;
97        }
98
99        // Bound viewport position so it doesn't go off-screen
100        viewport.physical_position = viewport
101            .physical_position
102            .min(window_size - viewport.physical_size);
103
104        // Viewport size controls
105        if input.pressed(KeyCode::KeyI) {
106            viewport.physical_size.y = viewport.physical_size.y.saturating_sub(uspeed);
107        }
108        if input.pressed(KeyCode::KeyK) {
109            viewport.physical_size.y += uspeed;
110        }
111        if input.pressed(KeyCode::KeyJ) {
112            viewport.physical_size.x = viewport.physical_size.x.saturating_sub(uspeed);
113        }
114        if input.pressed(KeyCode::KeyL) {
115            viewport.physical_size.x += uspeed;
116        }
117
118        // Bound viewport size so it doesn't go off-screen
119        viewport.physical_size = viewport
120            .physical_size
121            .min(window_size - viewport.physical_position)
122            .max(UVec2::new(20, 20));
123    }
124}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#202)

#### pub fn [clamp](#method.clamp)(self, min: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), max: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Component-wise clamping of values, similar to [`u32::clamp`](https://doc.rust-lang.org/nightly/std/primitive.u32.html#method.clamp "method u32::clamp").

Each element in `min` must be less-or-equal to the corresponding element in `max`.

##### Panics

Will panic if `min` is greater than `max` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#212)

#### pub fn [min\_element](#method.min_element)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the horizontal minimum of `self`.

In other words this computes `min(x, y, ..)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#222)

#### pub fn [max\_element](#method.max_element)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the horizontal maximum of `self`.

In other words this computes `max(x, y, ..)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#231)

#### pub fn [min\_position](#method.min_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first minimum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#243)

#### pub fn [max\_position](#method.max_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first maximum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#256)

#### pub fn [element\_sum](#method.element_sum)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the sum of all elements of `self`.

In other words, this computes `self.x + self.y + ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#265)

#### pub fn [element\_product](#method.element_product)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the product of all elements of `self`.

In other words, this computes `self.x * self.y * ..`.

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/2d/tilemap\_chunk.rs ([line 33](../../../src/tilemap_chunk/tilemap_chunk.rs.html#33))

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

Hide additional examples

examples/2d/tilemap\_chunk\_orientation.rs ([line 45](../../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#45))

```rust
17fn setup(mut commands: Commands, assets: Res<AssetServer>) {
18    let chunk_size = UVec2::splat(8);
19    let tile_display_size = UVec2::splat(64);
20
21    // We'll use each possible orientation, one per column
22    let orientation = [
23        TileOrientation::Default,
24        TileOrientation::Rotate90,
25        TileOrientation::Rotate180,
26        TileOrientation::Rotate270,
27        TileOrientation::MirrorH,
28        TileOrientation::MirrorHRotate90,
29        TileOrientation::MirrorHRotate180,
30        TileOrientation::MirrorHRotate270,
31    ];
32
33    // Show different color/alpha on each row
34    let colors = [
35        Color::WHITE,
36        Color::linear_rgb(1.0, 0.0, 0.0),
37        Color::linear_rgb(0.0, 1.0, 0.0),
38        Color::linear_rgb(0.0, 0.0, 1.0),
39        Color::linear_rgba(1.0, 0.0, 0.0, 0.25),
40        Color::linear_rgba(0.0, 1.0, 0.0, 0.25),
41        Color::linear_rgba(0.0, 0.0, 1.0, 0.25),
42        Color::linear_rgba(1.0, 1.0, 1.0, 0.5),
43    ];
44
45    let tile_data = (0..chunk_size.element_product())
46        .map(|i| {
47            let row = i / 8;
48            let col = i % 8;
49            Some(TileData {
50                // Alternate tiles per row
51                tileset_index: (row % 2) as u16,
52                color: colors[row as usize],
53                // Last (top) row is invisible
54                visible: row != 7,
55                orientation: orientation[col as usize],
56            })
57        })
58        .collect();
59
60    commands.spawn((
61        TilemapChunk {
62            chunk_size,
63            tile_display_size,
64            tileset: assets
65                .load_builder()
66                .with_settings(|settings: &mut ImageLoaderSettings| {
67                    // The tileset texture is expected to be an array of tile textures, so we tell the
68                    // `ImageLoader` that our texture is composed of 2 stacked tile images.
69                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 2 });
70                })
71                .load("textures/arrow.png"),
72            alpha_mode: AlphaMode2d::Blend,
73        },
74        TilemapChunkTileData(tile_data),
75    ));
76
77    commands.spawn(Camera2d);
78}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#276)

#### pub fn [cmpeq](#method.cmpeq)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `==` comparison for each element of `self` and `rhs`.

In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#287)

#### pub fn [cmpne](#method.cmpne)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `!=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#298)

#### pub fn [cmpge](#method.cmpge)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `>=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#309)

#### pub fn [cmpgt](#method.cmpgt)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `>` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#320)

#### pub fn [cmple](#method.cmple)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `<=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#331)

#### pub fn [cmplt](#method.cmplt)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `<` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#339)

#### pub fn [length\_squared](#method.length_squared)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Computes the squared length of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#353)

#### pub fn [manhattan\_distance](#method.manhattan_distance)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Computes the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.

##### Overflow

This method may overflow if the result is greater than [`u32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.u32.html#associatedconstant.MAX "associated constant u32::MAX").

See also [`checked_manhattan_distance`](../../prelude/struct.UVec2.html#method.checked_manhattan_distance "method bevy::prelude::UVec2::checked_manhattan_distance").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#364)

#### pub fn [checked\_manhattan\_distance](#method.checked_manhattan_distance)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

Computes the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.

This will returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the result is greater than [`u32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.u32.html#associatedconstant.MAX "associated constant u32::MAX").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#374)

#### pub fn [chebyshev\_distance](#method.chebyshev_distance)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Computes the [chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance) between two points.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#385)

#### pub fn [as\_vec2](#method.as_vec2)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Casts all elements of `self` to `f32`.

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/shader/compute\_shader\_game\_of\_life.rs ([line 65](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#65))

```rust
54fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
55    let mut image = Image::new_target_texture(SIZE.x, SIZE.y, TextureFormat::Rgba32Float, None);
56    image.asset_usage = RenderAssetUsages::RENDER_WORLD;
57    image.texture_descriptor.usage =
58        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
59    let image0 = images.add(image.clone());
60    let image1 = images.add(image);
61
62    commands.spawn((
63        Sprite {
64            image: image0.clone(),
65            custom_size: Some(SIZE.as_vec2()),
66            ..default()
67        },
68        Transform::from_scale(Vec3::splat(DISPLAY_FACTOR as f32)),
69    ));
70    commands.spawn(Camera2d);
71
72    commands.insert_resource(GameOfLifeImages {
73        texture_a: image0,
74        texture_b: image1,
75    });
76
77    commands.insert_resource(GameOfLifeUniforms {
78        alive_color: LinearRgba::RED,
79    });
80}
```

Hide additional examples

examples/2d/dynamic\_mip\_generation.rs ([line 649](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#649))

```rust
629fn spawn_mip_level_views(
630    commands: &mut Commands,
631    app_status: &AppStatus,
632    app_assets: &AppAssets,
633    windows_query: &Query<&Window, With<PrimaryWindow>>,
634    single_mip_level_materials: &mut Assets<SingleMipLevelMaterial>,
635    image_handle: &Handle<Image>,
636) {
637    let window_size = windows_query.iter().next().unwrap().size();
638
639    // Calculate the placement of the column of mipmap levels.
640    let max_slice_size = app_status.max_mip_slice_size(window_size);
641    let y_origin = app_status.vertical_mip_slice_origin(window_size);
642    let y_spacing = app_status.vertical_mip_slice_spacing(window_size);
643    let x_origin = app_status.horizontal_mip_slice_origin(window_size);
644
645    for (mip_level, mip_size) in MipmapSizeIterator::new(app_status).enumerate() {
646        let y_center = y_origin - y_spacing * mip_level as f32;
647
648        // Size each image to fit its container, preserving aspect ratio.
649        let mut slice_size = mip_size.as_vec2();
650        let ratios = max_slice_size / slice_size;
651        let slice_scale = ratios.x.min(ratios.y).min(1.0);
652        slice_size *= slice_scale;
653
654        // Spawn the image. Use the `SingleMipLevelMaterial` with its custom
655        // shader so that only the mip level in question is displayed.
656        commands.spawn((
657            Mesh2d(app_assets.rectangle.clone()),
658            MeshMaterial2d(single_mip_level_materials.add(SingleMipLevelMaterial {
659                mip_level: mip_level as u32,
660                texture: image_handle.clone(),
661            })),
662            Transform::from_xyz(x_origin, y_center, 0.0).with_scale(slice_size.extend(1.0)),
663            ImageView,
664        ));
665
666        // Display a label to the side.
667        commands.spawn((
668            Text2d::new(format!(
669                "Level {}\n{}×{}",
670                mip_level, mip_size.x, mip_size.y
671            )),
672            app_assets.text_font.clone(),
673            TextLayout::justify(Justify::Center),
674            Text2dShadow::default(),
675            Transform::from_xyz(x_origin - max_slice_size.x * 0.5 - 64.0, y_center, 0.0),
676            ImageView,
677        ));
678    }
679}
```

examples/ui/render\_ui\_to\_texture.rs ([line 195](../../../src/render_ui_to_texture/render_ui_to_texture.rs.html#195))

```rust
173fn drive_diegetic_pointer(
174    mut cursor_last: Local<Vec2>,
175    mut raycast: MeshRayCast,
176    rays: Res<RayMap>,
177    cubes: Query<&Mesh3d, With<Cube>>,
178    ui_camera: Query<&RenderTarget, With<Camera2d>>,
179    primary_window: Query<Entity, With<PrimaryWindow>>,
180    windows: Query<(Entity, &Window)>,
181    images: Res<Assets<Image>>,
182    manual_texture_views: Res<ManualTextureViews>,
183    mut window_events: MessageReader<WindowEvent>,
184    mut pointer_inputs: MessageWriter<PointerInput>,
185) -> Result {
186    // Get the size of the texture, so we can convert from dimensionless UV coordinates that span
187    // from 0 to 1, to pixel coordinates.
188    let target = ui_camera
189        .single()?
190        .normalize(primary_window.single().ok())
191        .unwrap();
192    let target_info = target
193        .get_render_target_info(windows, &images, &manual_texture_views)
194        .unwrap();
195    let size = target_info.physical_size.as_vec2();
196
197    // Find raycast hits and update the virtual pointer.
198    let raycast_settings = MeshRayCastSettings {
199        visibility: RayCastVisibility::VisibleInView,
200        filter: &|entity| cubes.contains(entity),
201        early_exit_test: &|_| false,
202    };
203    for (_id, ray) in rays.iter() {
204        for (_cube, hit) in raycast.cast_ray(*ray, &raycast_settings) {
205            let position = size * hit.uv.unwrap();
206            if position != *cursor_last {
207                pointer_inputs.write(PointerInput::new(
208                    CUBE_POINTER_ID,
209                    Location {
210                        target: target.clone(),
211                        position,
212                    },
213                    PointerAction::Move {
214                        delta: position - *cursor_last,
215                    },
216                ));
217                *cursor_last = position;
218            }
219        }
220    }
221
222    // Pipe pointer button presses to the virtual pointer on the UI texture.
223    for window_event in window_events.read() {
224        if let WindowEvent::MouseButtonInput(input) = window_event {
225            let button = match input.button {
226                MouseButton::Left => PointerButton::Primary,
227                MouseButton::Right => PointerButton::Secondary,
228                MouseButton::Middle => PointerButton::Middle,
229                _ => continue,
230            };
231            let action = match input.state {
232                ButtonState::Pressed => PointerAction::Press(button),
233                ButtonState::Released => PointerAction::Release(button),
234            };
235            pointer_inputs.write(PointerInput::new(
236                CUBE_POINTER_ID,
237                Location {
238                    target: target.clone(),
239                    position: *cursor_last,
240                },
241                action,
242            ));
243        }
244    }
245
246    Ok(())
247}
```

examples/2d/2d\_viewport\_to\_world.rs ([line 82](../../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#82))

```rust
42fn controls(
43    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
44    window: Single<&Window>,
45    input: Res<ButtonInput<KeyCode>>,
46    time: Res<Time<Fixed>>,
47) {
48    let (mut camera, mut transform, mut projection) = camera_query.into_inner();
49
50    let fspeed = 600.0 * time.delta_secs();
51    let uspeed = fspeed as u32;
52    let window_size = window.resolution.physical_size();
53
54    // Camera movement controls
55    if input.pressed(KeyCode::ArrowUp) {
56        transform.translation.y += fspeed;
57    }
58    if input.pressed(KeyCode::ArrowDown) {
59        transform.translation.y -= fspeed;
60    }
61    if input.pressed(KeyCode::ArrowLeft) {
62        transform.translation.x -= fspeed;
63    }
64    if input.pressed(KeyCode::ArrowRight) {
65        transform.translation.x += fspeed;
66    }
67
68    // Camera zoom controls
69    if let Projection::Orthographic(projection2d) = &mut *projection {
70        if input.pressed(KeyCode::Comma) {
71            projection2d.scale *= powf(4.0f32, time.delta_secs());
72        }
73
74        if input.pressed(KeyCode::Period) {
75            projection2d.scale *= powf(0.25f32, time.delta_secs());
76        }
77    }
78
79    if let Some(viewport) = camera.viewport.as_mut() {
80        // Reset viewport size on window resize
81        if viewport.physical_size.x > window_size.x || viewport.physical_size.y > window_size.y {
82            viewport.physical_size = (window_size.as_vec2() * 0.75).as_uvec2();
83        }
84
85        // Viewport movement controls
86        if input.pressed(KeyCode::KeyW) {
87            viewport.physical_position.y = viewport.physical_position.y.saturating_sub(uspeed);
88        }
89        if input.pressed(KeyCode::KeyS) {
90            viewport.physical_position.y += uspeed;
91        }
92        if input.pressed(KeyCode::KeyA) {
93            viewport.physical_position.x = viewport.physical_position.x.saturating_sub(uspeed);
94        }
95        if input.pressed(KeyCode::KeyD) {
96            viewport.physical_position.x += uspeed;
97        }
98
99        // Bound viewport position so it doesn't go off-screen
100        viewport.physical_position = viewport
101            .physical_position
102            .min(window_size - viewport.physical_size);
103
104        // Viewport size controls
105        if input.pressed(KeyCode::KeyI) {
106            viewport.physical_size.y = viewport.physical_size.y.saturating_sub(uspeed);
107        }
108        if input.pressed(KeyCode::KeyK) {
109            viewport.physical_size.y += uspeed;
110        }
111        if input.pressed(KeyCode::KeyJ) {
112            viewport.physical_size.x = viewport.physical_size.x.saturating_sub(uspeed);
113        }
114        if input.pressed(KeyCode::KeyL) {
115            viewport.physical_size.x += uspeed;
116        }
117
118        // Bound viewport size so it doesn't go off-screen
119        viewport.physical_size = viewport
120            .physical_size
121            .min(window_size - viewport.physical_position)
122            .max(UVec2::new(20, 20));
123    }
124}
125
126fn setup(
127    mut commands: Commands,
128    mut meshes: ResMut<Assets<Mesh>>,
129    mut materials: ResMut<Assets<ColorMaterial>>,
130    window: Single<&Window>,
131) {
132    let window_size = window.resolution.physical_size().as_vec2();
133
134    // Initialize centered, non-window-filling viewport
135    commands.spawn((
136        Camera2d,
137        Camera {
138            viewport: Some(Viewport {
139                physical_position: (window_size * 0.125).as_uvec2(),
140                physical_size: (window_size * 0.75).as_uvec2(),
141                ..default()
142            }),
143            ..default()
144        },
145    ));
146
147    // Create a minimal UI explaining how to interact with the example
148    commands.spawn((
149        Text::new(
150            "Move the mouse to see the circle follow your cursor.\n\
151                    Use the arrow keys to move the camera.\n\
152                    Use the comma and period keys to zoom in and out.\n\
153                    Use the WASD keys to move the viewport.\n\
154                    Use the IJKL keys to resize the viewport.",
155        ),
156        Node {
157            position_type: PositionType::Absolute,
158            top: px(12),
159            left: px(12),
160            ..default()
161        },
162    ));
163
164    // Add mesh to make camera movement visible
165    commands.spawn((
166        Mesh2d(meshes.add(Rectangle::new(40.0, 20.0))),
167        MeshMaterial2d(materials.add(Color::from(GREEN))),
168    ));
169
170    // Add background to visualize viewport bounds
171    commands.spawn((
172        Mesh2d(meshes.add(Rectangle::new(50000.0, 50000.0))),
173        MeshMaterial2d(materials.add(Color::linear_rgb(0.01, 0.01, 0.01))),
174        Transform::from_translation(Vec3::new(0.0, 0.0, -200.0)),
175    ));
176}
```

examples/testbed/2d.rs ([line 507](../../../src/testbed_2d/2d.rs.html#507))

```rust
451    pub fn setup(
452        mut commands: Commands,
453        mut textures: ResMut<Assets<Image>>,
454        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
455    ) {
456        commands.spawn((Camera2d, DespawnOnExit(super::Scene::TextureAtlasBuilder)));
457
458        for (i, padding) in [UVec2::ZERO, PADDING_SIZE].into_iter().enumerate() {
459            // generate solid red green and blue and yellow images
460            let images = [
461                [255, 0, 0, 255],
462                [0, 255, 0, 255],
463                [0, 0, 255, 255],
464                [255, 255, 0, 255],
465            ]
466            .map(|pixel| {
467                Image::new_fill(
468                    Extent3d {
469                        width: 28,
470                        height: 28,
471                        depth_or_array_layers: 1,
472                    },
473                    TextureDimension::D2,
474                    &pixel,
475                    TextureFormat::Rgba8UnormSrgb,
476                    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
477                )
478            });
479
480            let mut texture_atlas_builder = TextureAtlasBuilder::default();
481            texture_atlas_builder
482                .initial_size(ATLAS_SIZE)
483                .max_size(ATLAS_SIZE)
484                .padding(padding);
485            for image in &images {
486                texture_atlas_builder.add_texture(None, image);
487            }
488
489            let (atlas_layout, _, atlas_texture) = texture_atlas_builder.build().expect(
490                "The images are 28 pixels square, so they should fit with 4 pixels left over",
491            );
492            let atlas_layout = texture_atlases.add(atlas_layout);
493
494            let mut nearest_atlas_image = atlas_texture.clone();
495            nearest_atlas_image.sampler = ImageSampler::nearest();
496
497            let atlas_handle = textures.add(atlas_texture);
498            let nearest_atlas_handle = textures.add(nearest_atlas_image);
499
500            let position = ((2. * i as f32 - 1.) * (0.625 * ATLAS_SIZE.x as f32 * ATLAS_SCALE))
501                .round()
502                * Vec3::X;
503
504            commands.spawn((
505                Sprite {
506                    image: nearest_atlas_handle,
507                    custom_size: Some(ATLAS_SIZE.as_vec2() * ATLAS_SCALE),
508                    ..default()
509                },
510                Anchor::BOTTOM_CENTER,
511                ShowAabbGizmo {
512                    color: Some(Color::WHITE),
513                },
514                DespawnOnExit(super::Scene::TextureAtlasBuilder),
515                Transform::from_translation(position),
516            ));
517
518            for (index, anchor) in [
519                Anchor::BOTTOM_RIGHT,
520                Anchor::BOTTOM_LEFT,
521                Anchor::TOP_LEFT,
522                Anchor::TOP_RIGHT,
523            ]
524            .into_iter()
525            .enumerate()
526            {
527                commands.spawn((
528                    Sprite {
529                        image: atlas_handle.clone(),
530                        texture_atlas: Some(TextureAtlas {
531                            layout: atlas_layout.clone(),
532                            index,
533                        }),
534                        custom_size: Some(IMAGE_SIZE.as_vec2() * IMAGE_SCALE),
535                        ..default()
536                    },
537                    Transform::from_translation(
538                        position
539                            + -2.
540                                * IMAGE_SCALE
541                                * (Vec3::Y * IMAGE_SIZE.y as f32 + anchor.as_vec().extend(0.)),
542                    ),
543                    anchor,
544                    DespawnOnExit(super::Scene::TextureAtlasBuilder),
545                ));
546            }
547        }
548    }
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#392)

#### pub fn [as\_dvec2](#method.as_dvec2)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

Casts all elements of `self` to `f64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#399)

#### pub fn [as\_i8vec2](#method.as_i8vec2)(self) -> [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

Casts all elements of `self` to `i8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#406)

#### pub fn [as\_u8vec2](#method.as_u8vec2)(self) -> [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

Casts all elements of `self` to `u8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#413)

#### pub fn [as\_i16vec2](#method.as_i16vec2)(self) -> [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

Casts all elements of `self` to `i16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#420)

#### pub fn [as\_u16vec2](#method.as_u16vec2)(self) -> [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

Casts all elements of `self` to `u16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#427)

#### pub fn [as\_ivec2](#method.as_ivec2)(self) -> [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

Casts all elements of `self` to `i32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#434)

#### pub fn [as\_i64vec2](#method.as_i64vec2)(self) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Casts all elements of `self` to `i64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#441)

#### pub fn [as\_u64vec2](#method.as_u64vec2)(self) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Casts all elements of `self` to `u64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#448)

#### pub fn [as\_isizevec2](#method.as_isizevec2)(self) -> [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

Casts all elements of `self` to `isize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#455)

#### pub fn [as\_usizevec2](#method.as_usizevec2)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Casts all elements of `self` to `usize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#464)

#### pub const fn [checked\_add](#method.checked_add)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>

Returns a vector containing the wrapping addition of `self` and `rhs`.

In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#482)

#### pub const fn [checked\_sub](#method.checked_sub)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>

Returns a vector containing the wrapping subtraction of `self` and `rhs`.

In other words this computes `Some([self.x - rhs.x, self.y - rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#500)

#### pub const fn [checked\_mul](#method.checked_mul)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>

Returns a vector containing the wrapping multiplication of `self` and `rhs`.

In other words this computes `Some([self.x * rhs.x, self.y * rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#518)

#### pub const fn [checked\_div](#method.checked_div)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>

Returns a vector containing the wrapping division of `self` and `rhs`.

In other words this computes `Some([self.x / rhs.x, self.y / rhs.y, ..])` but returns `None` on any division by zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#536)

#### pub const fn [wrapping\_add](#method.wrapping_add)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the wrapping addition of `self` and `rhs`.

In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#548)

#### pub const fn [wrapping\_sub](#method.wrapping_sub)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the wrapping subtraction of `self` and `rhs`.

In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#560)

#### pub const fn [wrapping\_mul](#method.wrapping_mul)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the wrapping multiplication of `self` and `rhs`.

In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#572)

#### pub const fn [wrapping\_div](#method.wrapping_div)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the wrapping division of `self` and `rhs`.

In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#584)

#### pub const fn [saturating\_add](#method.saturating_add)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the saturating addition of `self` and `rhs`.

In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#596)

#### pub const fn [saturating\_sub](#method.saturating_sub)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the saturating subtraction of `self` and `rhs`.

In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#608)

#### pub const fn [saturating\_mul](#method.saturating_mul)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the saturating multiplication of `self` and `rhs`.

In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#620)

#### pub const fn [saturating\_div](#method.saturating_div)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the saturating division of `self` and `rhs`.

In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#632)

#### pub const fn [checked\_add\_signed](#method.checked_add_signed)(self, rhs: [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>

Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.

In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#650)

#### pub const fn [wrapping\_add\_signed](#method.wrapping_add_signed)(self, rhs: [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.

In other words this computes `[self.x.wrapping_add_signed(rhs.x), self.y.wrapping_add_signed(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#662)

#### pub const fn [saturating\_add\_signed](#method.saturating_add_signed)(self, rhs: [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a vector containing the saturating addition of `self` and signed vector `rhs`.

In other words this computes `[self.x.saturating_add_signed(rhs.x), self.y.saturating_add_signed(rhs.y), ..]`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#947)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#948)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#950)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#958)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#959)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#961)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#966)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#967)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#969)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1058)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1059)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1061)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1066)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1067)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1069)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1008)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1009)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1011)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1016)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1017)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1019)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#974)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#975)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#977)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1047)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1048)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1050)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1074)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1075)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1077)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#997)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#998)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1000)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1024)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1025)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1027)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#982)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#984)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#990)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#992)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1040)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1042)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1032)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1034)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1359)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1361)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [AsMutVectorParts](../../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

#### fn [as\_mut\_parts](../../render/render_resource/encase/vector/trait.AsMutVectorParts.html#tymethod.as_mut_parts)(&mut self) -> &mut \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1352)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1354)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [AsRefVectorParts](../../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

#### fn [as\_ref\_parts](../../render/render_resource/encase/vector/trait.AsRefVectorParts.html#tymethod.as_ref_parts)(&self) -> &\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1425)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1426)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1428)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1436)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1437)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1439)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1444)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1445)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1447)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1583)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1584)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1586)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1591)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1592)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1594)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1452)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1453)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1455)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1572)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1573)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1575)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1599)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1600)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1602)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1460)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1462)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1467)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1469)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1614)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1616)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1607)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1609)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1474)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1475)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1477)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1485)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1486)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1488)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1493)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1494)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1496)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1632)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1633)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1635)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1640)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1641)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1643)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1501)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1502)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1504)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1621)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1622)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1624)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1648)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1649)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1651)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1509)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1511)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1516)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1518)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1663)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1665)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1656)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1658)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1523)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1524)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1526)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1534)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1535)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1537)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1542)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1543)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1545)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1681)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1682)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1684)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1689)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1690)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1692)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1550)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1551)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1553)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1670)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1671)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1673)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1697)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1698)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1700)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1558)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1560)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1565)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1567)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1712)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1714)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1705)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1707)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#22)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#22)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#22)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [CreateFrom](../../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [FromVectorParts](../../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](../../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

#### fn [create\_from](../../render/render_resource/encase/internal/trait.CreateFrom.html#tymethod.create_from)<B>(reader: &mut [Reader](../../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where B: [BufferRef](../../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2672)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2673)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#670)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#672)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1198)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Deserialize expects a sequence of 2 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1198)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2666)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2667)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#677)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#678)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#680)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#688)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#689)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#691)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#696)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#697)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#699)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#788)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#789)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#791)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#796)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#797)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#799)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#738)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#739)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#741)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#746)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#747)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#749)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#704)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#705)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#707)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#777)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#778)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#780)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#804)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#805)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#807)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#727)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#728)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#730)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#754)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#755)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#757)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#712)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#714)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#720)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#722)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#770)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#772)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#762)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#764)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#22)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2695)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2697)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: ([u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2786)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2788)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2709)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2711)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2716)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2718)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec2.rs.html#2000)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec2.rs.html#2002)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2688)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2690)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2916)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2918)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2723)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2725)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#890)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [RenderTarget](../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#890)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [RenderTarget](../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1058)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [WindowResolution](../../window/struct.WindowResolution.html "struct bevy::window::WindowResolution")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1059)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(res: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [WindowResolution](../../window/struct.WindowResolution.html "struct bevy::window::WindowResolution")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2681)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2683)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(a: \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### type [This](../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The type to convert into. [Read more](../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [from\_arg](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [from\_reflect](../../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [FromVectorParts](../../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

#### fn [from\_parts](../../render/render_resource/encase/vector/trait.FromVectorParts.html#tymethod.from_parts)(parts: \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [GetOwnership](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [ownership](../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [get\_type\_registration](../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [register\_type\_dependencies](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#22)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#22)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<\_\_H>(&self, state: [&mut \_\_H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where \_\_H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2643)

### impl [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2644)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

The returned type after indexing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2646)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2655)

### impl [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2657)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &mut <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [IntoReturn](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [into\_return](../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): 'into\_return,

Converts [`Self`](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#812)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#813)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#815)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#823)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#824)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#826)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#831)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#832)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#834)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#923)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#924)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#926)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#931)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#932)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#934)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#873)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#874)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#876)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#881)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#882)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#884)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#839)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#840)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#842)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#912)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#913)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#915)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#939)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#940)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#942)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#862)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#863)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#865)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#889)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#890)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#892)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#847)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#849)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#855)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#857)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#905)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#907)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#897)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#899)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1406)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1407)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1409)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1417)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1418)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1420)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#22)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#22)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [get\_represented\_type\_info](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [try\_apply](../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [reflect\_kind](../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [reflect\_ref](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [reflect\_owned](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>) -> [ReflectOwned](../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [try\_into\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [try\_as\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [try\_as\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [into\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [as\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [as\_partial\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#139)

#### fn [reflect\_hash](../../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#139)

#### fn [reflect\_partial\_eq](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [reflect\_partial\_cmp](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#139)

#### fn [debug](../../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#139)

#### fn [reflect\_clone](../../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](../../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](../../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](../../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](../../prelude/trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](../../prelude/trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#23)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1386)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1388-1390)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1396)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1398-1400)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [ReadFrom](../../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [AsMutVectorParts](../../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](../../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

#### fn [read\_from](../../render/render_resource/encase/internal/trait.ReadFrom.html#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](../../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](../../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [into\_any](../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [as\_any](../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [as\_any\_mut](../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [into\_reflect](../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [as\_reflect](../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [as\_reflect\_mut](../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [set](../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1217)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1218)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1220)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1228)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1229)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1231)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1236)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1237)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1239)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1328)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1329)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1331)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1336)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1337)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1339)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1278)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1279)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1281)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1286)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1287)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1289)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1244)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1245)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1247)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1317)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1318)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1320)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1344)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1345)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1347)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1267)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1268)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1270)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1294)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1295)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1297)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1252)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1254)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1260)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1262)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1310)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1312)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1302)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1304)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#707)

### impl [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#707)

#### type [Sampler](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html#associatedtype.Sampler) = UniformVec2<[UniformInt](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/int/struct.UniformInt.html "struct rand::distr::uniform::int::UniformInt")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>

The `UniformSampler` implementation supporting type `X`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1198)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Serialize as a sequence of 2 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1198)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [ShaderSize](../../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderSize](../../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#232)

#### const [SHADER\_SIZE](../../render/render_resource/trait.ShaderSize.html#associatedconstant.SHADER_SIZE): [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> = \_

Represents [WGSL Size](https://gpuweb.github.io/gpuweb/wgsl/#alignment-and-size) (equivalent to [`ShaderType::min_size`](../../render/render_resource/trait.ShaderType.html#method.min_size "associated function bevy::render::render_resource::ShaderType::min_size"))

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderSize](../../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#94)

#### fn [min\_size](../../render/render_resource/trait.ShaderType.html#method.min_size)() -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Represents the minimum size of `Self` (equivalent to [GPUBufferBindingLayout.minBindingSize](https://gpuweb.github.io/gpuweb/#dom-gpubufferbindinglayout-minbindingsize)) [Read more](../../render/render_resource/trait.ShaderType.html#method.min_size)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#103)

#### fn [size](../../render/render_resource/trait.ShaderType.html#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns the size of `Self` at runtime [Read more](../../render/render_resource/trait.ShaderType.html#method.size)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#206)

#### fn [assert\_uniform\_compat](../../render/render_resource/trait.ShaderType.html#method.assert_uniform_compat)()

Asserts that `Self` meets the requirements of the [uniform address space restrictions on stored values](https://gpuweb.github.io/gpuweb/wgsl/#address-spaces-uniform) and the [uniform address space layout constraints](https://gpuweb.github.io/gpuweb/wgsl/#address-space-layout-constraints) [Read more](../../render/render_resource/trait.ShaderType.html#method.assert_uniform_compat)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2573)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2574)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2576)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2514)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2515)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2517)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2522)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2523)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2525)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2756)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2757)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2759)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2764)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2765)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2767)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2584)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2585)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2587)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2592)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2593)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2595)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2756)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2757)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2759)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2764)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2765)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2767)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2584)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2585)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2587)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2592)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2593)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2595)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2756)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2757)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2759)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2764)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2765)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2767)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2584)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2585)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2587)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2592)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2593)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2595)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2756)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2757)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2759)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2764)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2765)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2767)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2584)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2585)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2587)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2592)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2593)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2595)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2584)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2585)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2587)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2592)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2593)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2595)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2756)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2757)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2759)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2764)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2765)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2767)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1730)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1731)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1733)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1738)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1739)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1741)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1828)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1829)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1831)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1836)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1837)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1839)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1926)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1927)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1929)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1934)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1935)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1937)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2024)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2025)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2027)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2032)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2033)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2035)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2122)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2123)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2125)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2130)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2131)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2133)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2220)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2221)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2223)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2228)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2229)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2231)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2318)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2319)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2321)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2326)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2327)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2329)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2416)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2417)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2419)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2424)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2425)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2427)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2503)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2504)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2506)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2530)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2531)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2533)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2745)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2746)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2748)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2772)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2773)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2775)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2573)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2574)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2576)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2600)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2601)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2603)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2745)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2746)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2748)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2772)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2773)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2775)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2573)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2574)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2576)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2600)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2601)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2603)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2745)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2746)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2748)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2772)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2773)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2775)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2600)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2601)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2603)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2745)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2746)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2748)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2772)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2773)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2775)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2573)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2574)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2576)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2600)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2601)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2603)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2573)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2574)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2576)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2600)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2601)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2603)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2745)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2746)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2748)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2772)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2773)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2775)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1719)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1720)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1722)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1746)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1747)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1749)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1817)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1818)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1820)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1844)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1845)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1847)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1915)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1916)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1918)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1942)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1943)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1945)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2013)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2014)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2016)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2040)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2041)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2043)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2111)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2112)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2114)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2138)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2139)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2141)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2209)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2210)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2212)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2236)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2237)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2239)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2307)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2308)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2310)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2334)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2335)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2337)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2405)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2406)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2408)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2432)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2433)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2435)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1761)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1763)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1859)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1861)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1957)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1959)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2055)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2057)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2153)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2155)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2251)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2253)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2349)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2351)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2447)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2449)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1754)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1756)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1852)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1854)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1950)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1952)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2048)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2050)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2146)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2148)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2244)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2246)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2342)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2344)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2440)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2442)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2608)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2609)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2611)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2549)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2550)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2552)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2557)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2558)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2560)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2791)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2792)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2794)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2799)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2800)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2802)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2619)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2620)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2622)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2627)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2628)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2630)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2791)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2792)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2794)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2799)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2800)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2802)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2619)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2620)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2622)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2627)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2628)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2630)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2791)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2792)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2794)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2799)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2800)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2802)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2619)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2620)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2622)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2627)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2628)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2630)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2791)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2792)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2794)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2799)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2800)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2802)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2619)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2620)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2622)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2627)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2628)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2630)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2619)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2620)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2622)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2627)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2628)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2630)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2791)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2792)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2794)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2799)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2800)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2802)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1779)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1780)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1782)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1787)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1788)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1790)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1877)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1878)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1880)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1885)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1886)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1888)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1975)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1976)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1978)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1983)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1984)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1986)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2073)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2074)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2076)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2081)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2082)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2084)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2171)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2172)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2174)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2179)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2180)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2182)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2269)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2270)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2272)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2277)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2278)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2280)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2367)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2368)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2370)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2375)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2376)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2378)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2465)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2466)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2468)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2473)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2474)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2476)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2538)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2539)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2541)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2565)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2566)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2568)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2780)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2781)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2783)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2807)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2808)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2810)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2608)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2609)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2611)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2635)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2636)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2638)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2780)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2781)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2783)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2807)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2808)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2810)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2608)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2609)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2611)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2635)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2636)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2638)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2780)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2781)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2783)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2807)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2808)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2810)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2635)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2636)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2638)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2780)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2781)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2783)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2807)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2808)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec2.rs.html#2810)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2608)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2609)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2611)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2635)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2636)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec2.rs.html#2638)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2608)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2609)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2611)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2635)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2636)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2638)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2780)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2781)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2783)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2807)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2808)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2810)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1768)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1769)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1771)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1795)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1796)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1798)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1866)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1867)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1869)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1893)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1894)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1896)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1964)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1965)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1967)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1991)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1992)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1994)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2062)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2063)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2065)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2089)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2090)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2092)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2160)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2161)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2163)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2187)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2188)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2190)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2258)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2259)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2261)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2285)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2286)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2288)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2356)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2357)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2359)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2383)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2384)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2386)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2454)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2455)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2457)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2481)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2482)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2484)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1810)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1812)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1908)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1910)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2006)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2008)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2104)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2106)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2202)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2204)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2300)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2302)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2398)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2400)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2496)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2498)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1803)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1805)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1901)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1903)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1999)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2001)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2097)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2099)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2195)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2197)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2293)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2295)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2391)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2393)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2489)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2491)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [field](../../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [field\_mut](../../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [field\_at](../../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [field\_at\_mut](../../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [name\_at](../../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [index\_of\_name](../../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [field\_len](../../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [iter\_fields](../../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [to\_dynamic\_struct](../../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#22)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1082)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1083)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1085)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1093)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1094)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1096)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1101)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1102)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1104)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1193)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1194)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1196)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1201)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1202)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1204)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1143)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1144)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1146)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1151)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1152)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1154)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1109)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1110)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1112)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1182)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1183)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1185)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1209)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1210)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1212)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1132)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1133)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1135)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1159)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1160)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1162)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1117)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1119)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1125)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1127)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1175)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1177)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1167)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1169)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1366)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1368-1370)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1376)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#1378-1380)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#580)

### impl [ToExtents](../../image/trait.ToExtents.html "trait bevy::image::ToExtents") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#581)

#### fn [to\_extents](../../image/trait.ToExtents.html#tymethod.to_extents)(self) -> [Extent3d](../../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d")

Converts this type to an [`Extent3d`](../../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2723)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2724)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2727)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2732)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2733)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2736)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2750)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2751)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2754)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2768)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2769)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2772)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2741)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2742)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2745)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2759)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2760)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2763)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2777)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")\> for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2778)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#2781)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), <[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2917)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2918)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec2.rs.html#2921)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2"), <[I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2745)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2746)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec2.rs.html#2749)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2"), <[U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2913)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2914)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec2.rs.html#2917)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2"), <[I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2743)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2744)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec2.rs.html#2747)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2"), <[U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2909)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2910)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec2.rs.html#2913)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"), <[IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2759)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2760)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec2.rs.html#2763)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2"), <[USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2911)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> for [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2912)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec2.rs.html#2915)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2"), <[ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [type\_path](../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [short\_type\_path](../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [type\_ident](../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [crate\_name](../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [module\_path](../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### fn [type\_info](../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#5)

### impl [Vec2Swizzles](../../prelude/trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#6)

#### type [Vec3](../../prelude/trait.Vec2Swizzles.html#associatedtype.Vec3) = [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#8)

#### type [Vec4](../../prelude/trait.Vec2Swizzles.html#associatedtype.Vec4) = [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#11)

#### fn [xx](../../prelude/trait.Vec2Swizzles.html#tymethod.xx)(self) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#19)

#### fn [yx](../../prelude/trait.Vec2Swizzles.html#tymethod.yx)(self) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#27)

#### fn [yy](../../prelude/trait.Vec2Swizzles.html#tymethod.yy)(self) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#35)

#### fn [xxx](../../prelude/trait.Vec2Swizzles.html#tymethod.xxx)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#40)

#### fn [xxy](../../prelude/trait.Vec2Swizzles.html#tymethod.xxy)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#45)

#### fn [xyx](../../prelude/trait.Vec2Swizzles.html#tymethod.xyx)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#50)

#### fn [xyy](../../prelude/trait.Vec2Swizzles.html#tymethod.xyy)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#55)

#### fn [yxx](../../prelude/trait.Vec2Swizzles.html#tymethod.yxx)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#60)

#### fn [yxy](../../prelude/trait.Vec2Swizzles.html#tymethod.yxy)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#65)

#### fn [yyx](../../prelude/trait.Vec2Swizzles.html#tymethod.yyx)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#70)

#### fn [yyy](../../prelude/trait.Vec2Swizzles.html#tymethod.yyy)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#75)

#### fn [xxxx](../../prelude/trait.Vec2Swizzles.html#tymethod.xxxx)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#80)

#### fn [xxxy](../../prelude/trait.Vec2Swizzles.html#tymethod.xxxy)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#85)

#### fn [xxyx](../../prelude/trait.Vec2Swizzles.html#tymethod.xxyx)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#90)

#### fn [xxyy](../../prelude/trait.Vec2Swizzles.html#tymethod.xxyy)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#95)

#### fn [xyxx](../../prelude/trait.Vec2Swizzles.html#tymethod.xyxx)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#100)

#### fn [xyxy](../../prelude/trait.Vec2Swizzles.html#tymethod.xyxy)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#105)

#### fn [xyyx](../../prelude/trait.Vec2Swizzles.html#tymethod.xyyx)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#110)

#### fn [xyyy](../../prelude/trait.Vec2Swizzles.html#tymethod.xyyy)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#115)

#### fn [yxxx](../../prelude/trait.Vec2Swizzles.html#tymethod.yxxx)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#120)

#### fn [yxxy](../../prelude/trait.Vec2Swizzles.html#tymethod.yxxy)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#125)

#### fn [yxyx](../../prelude/trait.Vec2Swizzles.html#tymethod.yxyx)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#130)

#### fn [yxyy](../../prelude/trait.Vec2Swizzles.html#tymethod.yxyy)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#135)

#### fn [yyxx](../../prelude/trait.Vec2Swizzles.html#tymethod.yyxx)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#140)

#### fn [yyxy](../../prelude/trait.Vec2Swizzles.html#tymethod.yyxy)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#145)

#### fn [yyyx](../../prelude/trait.Vec2Swizzles.html#tymethod.yyyx)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#150)

#### fn [yyyy](../../prelude/trait.Vec2Swizzles.html#tymethod.yyyy)(self) -> [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#10)

#### fn [xy](../../prelude/trait.Vec2Swizzles.html#method.xy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [WriteInto](../../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [AsRefVectorParts](../../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](../../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

#### fn [write\_into](../../render/render_resource/encase/internal/trait.WriteInto.html#tymethod.write_into)<B>(&self, writer: &mut [Writer](../../render/render_resource/encase/internal/struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](../../render/render_resource/encase/internal/trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#23)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

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

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#25-27)

### impl<T> [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#29)

#### fn [dyn\_eq](../../app/trait.DynEq.html#tymethod.dyn_eq)(&self, other: &(dyn [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method tests for `self` and `other` values to be equal. [Read more](../../app/trait.DynEq.html#tymethod.dyn_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#47-49)

### impl<T> [DynHash](../../ecs/label/trait.DynHash.html "trait bevy::ecs::label::DynHash") for T

where T: [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#51)

#### fn [dyn\_hash](../../ecs/label/trait.DynHash.html#tymethod.dyn_hash)(&self, state: &mut dyn [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"))

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](../../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](../../prelude/trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](../../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](../../prelude/trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](../../prelude/trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Compare self to `key` and return `true` if they are equal.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#151-154)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#156)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

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

### impl<T> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#272)

### impl<S> [GetField](../../prelude/trait.GetField.html "trait bevy::prelude::GetField") for S

where S: [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#273)

#### fn [get\_field](../../prelude/trait.GetField.html#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#278)

#### fn [get\_field\_mut](../../prelude/trait.GetField.html#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](../../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](../../prelude/trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](../../prelude/trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](../../prelude/trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](../../prelude/trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.path_mut)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/gpu_array_buffer.rs.html#20)

### impl<T> [GpuArrayBufferable](../../render/render_resource/trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable") for T

where T: [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](../../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize") + [WriteInto](../../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

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

[Source](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/src/num_traits/lib.rs.html#143-144)

### impl<T, Rhs> [NumAssignOps](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.NumAssignOps.html "trait num_traits::NumAssignOps")<Rhs> for T

where T: [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<Rhs> + [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<Rhs> + [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<Rhs> + [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<Rhs> + [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<Rhs>,

[Source](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/src/num_traits/lib.rs.html#110-115)

### impl<T, Rhs, Output> [NumOps](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.NumOps.html "trait num_traits::NumOps")<Rhs, Output> for T

where T: [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<Rhs, Output = Output> + [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<Rhs, Output = Output> + [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<Rhs, Output = Output> + [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<Rhs, Output = Output> + [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<Rhs, Output = Output>,

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

[Source](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/src/num_traits/lib.rs.html#133)

### impl<T, Base> [RefNum](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.RefNum.html "trait num_traits::RefNum")<Base> for T

where T: [NumOps](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.NumOps.html "trait num_traits::NumOps")<Base, Base> + for<'r> [NumOps](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.NumOps.html "trait num_traits::NumOps")<[&'r Base](https://doc.rust-lang.org/nightly/std/primitive.reference.html), Base>,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/distr/uniform.rs.html#401-403)

### impl<Borrowed> [SampleBorrow](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleBorrow.html "trait rand::distr::uniform::SampleBorrow")<Borrowed> for Borrowed

where Borrowed: [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform"),

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/distr/uniform.rs.html#406)

#### fn [borrow](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleBorrow.html#tymethod.borrow)(&self) -> [&Borrowed](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. See [`Borrow::borrow`](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow "method core::borrow::Borrow::borrow")

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

### impl<T> [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#method.clone_into)

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

### impl<T> [ToString](../../prelude/trait.ToString.html "trait bevy::prelude::ToString") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2902)

#### fn [to\_string](../../prelude/trait.ToString.html#tymethod.to_string)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Converts the given value to a `String`. [Read more](../../prelude/trait.ToString.html#tymethod.to_string)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}