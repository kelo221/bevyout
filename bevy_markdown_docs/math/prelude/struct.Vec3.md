[bevy](../../index.html)::[math](../index.html)::[prelude](index.html)

# Struct Vec3 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#28)

```rust
#[repr(C)]pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
```

A 3-dimensional vector.

## Fields

`x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)``y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)``z: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#34)

### impl [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#36)

#### pub const [ZERO](#associatedconstant.ZERO): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

All zeroes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#39)

#### pub const [ONE](#associatedconstant.ONE): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

All ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#42)

#### pub const [NEG\_ONE](#associatedconstant.NEG_ONE): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

All negative ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#45)

#### pub const [MIN](#associatedconstant.MIN): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

All `f32::MIN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#48)

#### pub const [MAX](#associatedconstant.MAX): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

All `f32::MAX`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#51)

#### pub const [NAN](#associatedconstant.NAN): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

All `f32::NAN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#54)

#### pub const [INFINITY](#associatedconstant.INFINITY): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

All `f32::INFINITY`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#57)

#### pub const [NEG\_INFINITY](#associatedconstant.NEG_INFINITY): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

All `f32::NEG_INFINITY`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#60)

#### pub const [X](#associatedconstant.X): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

A unit vector pointing along the positive X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#63)

#### pub const [Y](#associatedconstant.Y): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

A unit vector pointing along the positive Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#66)

#### pub const [Z](#associatedconstant.Z): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

A unit vector pointing along the positive Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#69)

#### pub const [NEG\_X](#associatedconstant.NEG_X): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

A unit vector pointing along the negative X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#72)

#### pub const [NEG\_Y](#associatedconstant.NEG_Y): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

A unit vector pointing along the negative Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#75)

#### pub const [NEG\_Z](#associatedconstant.NEG_Z): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

A unit vector pointing along the negative Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#78)

#### pub const [AXES](#associatedconstant.AXES): \[[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The unit axes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#81)

#### pub const [USES\_CORE\_SIMD](#associatedconstant.USES_CORE_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec3 uses Rust Portable SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#83)

#### pub const [USES\_NEON](#associatedconstant.USES_NEON): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec3 uses Arm NEON

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#85)

#### pub const [USES\_SCALAR\_MATH](#associatedconstant.USES_SCALAR_MATH): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

Vec3 uses scalar math

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#87)

#### pub const [USES\_SSE2](#associatedconstant.USES_SSE2): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec3 uses Intel SSE2

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#89)

#### pub const [USES\_WASM\_SIMD](#associatedconstant.USES_WASM_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec3 uses WebAssembly 128-bit SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#91)

#### pub const [USES\_WASM32\_SIMD](#associatedconstant.USES_WASM32_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

👎Deprecated since 0.31.0:

Renamed to USES\_WASM\_SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#96)

#### pub const fn [new](#method.new)(x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Creates a new vector.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/usage/debug\_frustum\_culling.rs ([line 100](../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#100))

```rust
100const FREE_CAMERA_START_TARGET: Vec3 = Vec3::new(7., 1.5, 0.);
101
102fn setup(
103    mut commands: Commands,
104    windows: Query<&Window>,
105    mut config_store: ResMut<GizmoConfigStore>,
106    mut meshes: ResMut<Assets<Mesh>>,
107    mut materials: ResMut<Assets<StandardMaterial>>,
108) -> Result {
109    let window = windows.single()?;
110    // The camera that the user controls to observe the scene.
111    let free_camera = commands
112        .spawn((
113            Camera3d::default(),
114            FREE_CAMERA_START_TRANSFORM.looking_at(FREE_CAMERA_START_TARGET, Vec3::Y),
115            FreeCamera::default(),
116        ))
117        .id();
118
119    // The camera that we want to debug frustum culling for. This will be rendered
120    // as a picture-in-picture in the lower right ninth of the screen.
121    let my_camera = commands
122        .spawn((
123            Camera3d::default(),
124            Transform::from_xyz(0., 1.5, 0.).looking_at(Vec3::new(1.0, 1.5, 0.), Vec3::Y),
125            Camera {
126                order: 1,
127                // The camera-to-debug's view will be in the lower right ninth of the screen.
128                viewport: Some(Viewport {
129                    physical_position: window.physical_size() * 2 / 3,
130                    physical_size: window.physical_size() / 3,
131                    ..default()
132                }),
133                // Do not write the free camera's view rendering back into the P-I-P
134                msaa_writeback: MsaaWriteback::Off,
135                ..default()
136            },
137            MyCamera,
138        ))
139        .id();
140
141    // Instructions placed on top of the free_camera view
142    commands.spawn((
143        UiTargetCamera(free_camera),
144        Node {
145            width: percent(100),
146            height: percent(100),
147            ..default()
148        },
149        children![(
150            Text::new(
151                "This example utilizes free camera controls i.e. move with WASD and mouse grab to change orientation.\n\
152                Press '1' to move the free camera to where MyCamera is, matching its view frustum.\n\
153                Press '2' to move the free camera to its initial position in the example.",
154            ),
155            Node {
156                position_type: PositionType::Absolute,
157                top: px(12),
158                left: px(12),
159                ..default()
160            },
161        )]
162    ));
163    // Label for the picture-in-picture view of MyCamera
164    commands.spawn((
165        UiTargetCamera(my_camera),
166        Node {
167            width: percent(100),
168            height: percent(100),
169            ..default()
170        },
171        children![(
172            Text::new("View of MyCamera"),
173            Node {
174                position_type: PositionType::Absolute,
175                bottom: px(12),
176                right: px(100),
177                ..default()
178            },
179        )],
180    ));
181
182    // Green Floor Plane
183    commands.spawn((
184        Mesh3d(
185            meshes.add(
186                Plane3d::default()
187                    .mesh()
188                    .size(SHAPE_RING_RADIUS * 4., SHAPE_RING_RADIUS * 4.),
189            ),
190        ),
191        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
192    ));
193    // Blue Wall Plane
194    commands.spawn((
195        Mesh3d(meshes.add(Plane3d::default().mesh().size(5., 5.))),
196        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.5))),
197        Transform::from_xyz(20., 2.5, 10.).with_rotation(Quat::from_rotation_z(PI / 2.)),
198    ));
199    // Light
200    commands.spawn((
201        PointLight {
202            shadow_maps_enabled: true,
203            ..default()
204        },
205        Transform::from_xyz(0.0, 10.0, 0.0),
206    ));
207
208    // Configure all AABB's to have a default color of red
209    let (_, aabb_gizmo_config) = config_store.config_mut::<AabbGizmoConfigGroup>();
210    aabb_gizmo_config.default_color = Some(Color::LinearRgba(LinearRgba::RED));
211
212    // Configure the shapes on the ring that will have their AABB's drawn and updated
213    let white_matl = materials.add(Color::WHITE);
214    let shapes = [
215        meshes.add(Cuboid {
216            half_size: Vec3::new(2., 0.5, 1.),
217        }),
218        meshes.add(Tetrahedron {
219            vertices: [
220                Vec3::new(3., 4., 3.),
221                Vec3::new(-0.5, 4., -0.5),
222                Vec3::new(-0.5, -0.5, 3.),
223                Vec3::new(3., -0.5, -0.5),
224            ],
225        }),
226        meshes.add(Cylinder {
227            radius: 0.1,
228            half_height: 1.5,
229        }),
230        meshes.add(Cuboid {
231            half_size: Vec3::new(1., 0.1, 2.),
232        }),
233        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
234    ];
235    let shapes_len = shapes.len() as f32;
236    let mut shape_ring = commands.spawn((Transform::default(), Visibility::default(), ShapeRing));
237    for (i, shape) in shapes.into_iter().enumerate() {
238        // Space the shapes out evenly along the ring
239        let shape_angle = i as f32 * 2. * PI / shapes_len;
240        let (s, c) = ops::sin_cos(shape_angle);
241        let (x, z) = (SHAPE_RING_RADIUS * c, SHAPE_RING_RADIUS * s);
242        shape_ring.with_child((
243            Mesh3d(shape),
244            MeshMaterial3d(white_matl.clone()),
245            Transform::from_xyz(x, 1.5, z).with_rotation(Quat::from_rotation_x(-PI / 4.)),
246            MyShape,
247        ));
248    }
249
250    // Configure the shape that peeks out of the wall plane
251    let wall_shape = meshes.add(Torus::default());
252    commands.spawn((
253        Mesh3d(wall_shape),
254        MeshMaterial3d(white_matl.clone()),
255        Transform::from_xyz(25., 1.5, 12.5).with_rotation(Quat::from_rotation_x(-PI / 4.)),
256        WallShape,
257    ));
258
259    Ok(())
260}
```

Hide additional examples

examples/showcase/breakout.rs ([line 21](../../../src/breakout/breakout.rs.html#21))

```rust
21const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
22const BALL_DIAMETER: f32 = 30.;
23const BALL_SPEED: f32 = 400.0;
24const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
25
26const WALL_THICKNESS: f32 = 10.0;
27// x coordinates
28const LEFT_WALL: f32 = -450.;
29const RIGHT_WALL: f32 = 450.;
30// y coordinates
31const BOTTOM_WALL: f32 = -300.;
32const TOP_WALL: f32 = 300.;
33
34const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
35// These values are exact
36const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
37const GAP_BETWEEN_BRICKS: f32 = 5.0;
38// These values are lower bounds, as the number of bricks is computed
39const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
40const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;
41
42const SCOREBOARD_FONT_SIZE: FontSize = FontSize::Px(33.0);
43const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
44
45const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
46const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
47const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
48const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
49const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
50const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
51const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
52
53fn main() {
54    App::new()
55        .add_plugins(DefaultPlugins)
56        .add_plugins(
57            stepping::SteppingPlugin::default()
58                .add_schedule(Update)
59                .at(percent(35), percent(50)),
60        )
61        .insert_resource(Score(0))
62        .insert_resource(ClearColor(BACKGROUND_COLOR))
63        .add_systems(Startup, setup)
64        // Add our simulation systems to the update schedule
65        // which is called once per frame.
66        .add_systems(
67            Update,
68            (apply_velocity, move_paddle, check_for_collisions)
69                // `chain`ing systems together runs them in order
70                .chain(),
71        )
72        .add_systems(Update, update_scoreboard)
73        .add_observer(play_collision_sound)
74        .run();
75}
76
77#[derive(Component)]
78struct Paddle;
79
80#[derive(Component)]
81struct Ball;
82
83#[derive(Component, Deref, DerefMut)]
84struct Velocity(Vec2);
85
86#[derive(Event)]
87struct BallCollided;
88
89#[derive(Component)]
90struct Brick;
91
92#[derive(Resource, Deref)]
93struct CollisionSound(Handle<AudioSource>);
94
95// Default must be implemented to define this as a required component for the Wall component below
96#[derive(Component, Default)]
97struct Collider;
98
99// This is a collection of the components that define a "Wall" in our game
100#[derive(Component)]
101#[require(Sprite, Transform, Collider)]
102struct Wall;
103
104/// Which side of the arena is this wall located on?
105enum WallLocation {
106    Left,
107    Right,
108    Bottom,
109    Top,
110}
111
112impl WallLocation {
113    /// Location of the *center* of the wall, used in `transform.translation()`
114    fn position(&self) -> Vec2 {
115        match self {
116            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
117            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
118            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
119            WallLocation::Top => Vec2::new(0., TOP_WALL),
120        }
121    }
122
123    /// (x, y) dimensions of the wall, used in `transform.scale()`
124    fn size(&self) -> Vec2 {
125        let arena_height = TOP_WALL - BOTTOM_WALL;
126        let arena_width = RIGHT_WALL - LEFT_WALL;
127        // Make sure we haven't messed up our constants
128        assert!(arena_height > 0.0);
129        assert!(arena_width > 0.0);
130
131        match self {
132            WallLocation::Left | WallLocation::Right => {
133                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
134            }
135            WallLocation::Bottom | WallLocation::Top => {
136                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
137            }
138        }
139    }
140}
141
142impl Wall {
143    // This "builder method" allows us to reuse logic across our wall entities,
144    // making our code easier to read and less prone to bugs when we change the logic
145    // Notice the use of Sprite and Transform alongside Wall, overwriting the default values defined for the required components
146    fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
147        (
148            Wall,
149            Sprite::from_color(WALL_COLOR, Vec2::ONE),
150            Transform {
151                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
152                // This is used to determine the order of our sprites
153                translation: location.position().extend(0.0),
154                // The z-scale of 2D objects must always be 1.0,
155                // or their ordering will be affected in surprising ways.
156                // See https://github.com/bevyengine/bevy/issues/4149
157                scale: location.size().extend(1.0),
158                ..default()
159            },
160        )
161    }
162}
163
164// This resource tracks the game's score
165#[derive(Resource, Deref, DerefMut)]
166struct Score(usize);
167
168#[derive(Component)]
169struct ScoreboardUi;
170
171// Add the game's entities to our world
172fn setup(
173    mut commands: Commands,
174    mut meshes: ResMut<Assets<Mesh>>,
175    mut materials: ResMut<Assets<ColorMaterial>>,
176    asset_server: Res<AssetServer>,
177) {
178    // Camera
179    commands.spawn(Camera2d);
180
181    // Sound
182    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
183    commands.insert_resource(CollisionSound(ball_collision_sound));
184
185    // Paddle
186    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
187
188    commands.spawn((
189        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
190        Transform {
191            translation: Vec3::new(0.0, paddle_y, 0.0),
192            scale: PADDLE_SIZE.extend(1.0),
193            ..default()
194        },
195        Paddle,
196        Collider,
197    ));
198
199    // Ball
200    commands.spawn((
201        Mesh2d(meshes.add(Circle::default())),
202        MeshMaterial2d(materials.add(BALL_COLOR)),
203        Transform::from_translation(BALL_STARTING_POSITION)
204            .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
205        Ball,
206        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
207    ));
208
209    // Scoreboard
210    commands.spawn((
211        Text::new("Score: "),
212        TextFont {
213            font_size: SCOREBOARD_FONT_SIZE,
214            ..default()
215        },
216        TextColor(TEXT_COLOR),
217        ScoreboardUi,
218        Node {
219            position_type: PositionType::Absolute,
220            top: SCOREBOARD_TEXT_PADDING,
221            left: SCOREBOARD_TEXT_PADDING,
222            ..default()
223        },
224        children![(
225            TextSpan::default(),
226            TextFont {
227                font_size: SCOREBOARD_FONT_SIZE,
228                ..default()
229            },
230            TextColor(SCORE_COLOR),
231        )],
232    ));
233
234    // Walls
235    commands.spawn(Wall::new(WallLocation::Left));
236    commands.spawn(Wall::new(WallLocation::Right));
237    commands.spawn(Wall::new(WallLocation::Bottom));
238    commands.spawn(Wall::new(WallLocation::Top));
239
240    // Bricks
241    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
242    let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
243    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;
244
245    assert!(total_width_of_bricks > 0.0);
246    assert!(total_height_of_bricks > 0.0);
247
248    // Given the space available, compute how many rows and columns of bricks we can fit
249    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
250    let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
251    let n_vertical_gaps = n_columns - 1;
252
253    // Because we need to round the number of columns,
254    // the space on the top and sides of the bricks only captures a lower bound, not an exact value
255    let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
256    let left_edge_of_bricks = center_of_bricks
257        // Space taken up by the bricks
258        - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
259        // Space taken up by the gaps
260        - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;
261
262    // In Bevy, the `translation` of an entity describes the center point,
263    // not its bottom-left corner
264    let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
265    let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;
266
267    for row in 0..n_rows {
268        for column in 0..n_columns {
269            let brick_position = Vec2::new(
270                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
271                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
272            );
273
274            // brick
275            commands.spawn((
276                Sprite {
277                    color: BRICK_COLOR,
278                    ..default()
279                },
280                Transform {
281                    translation: brick_position.extend(0.0),
282                    scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
283                    ..default()
284                },
285                Brick,
286                Collider,
287            ));
288        }
289    }
290}
```

examples/math/render\_primitives.rs ([line 157](../../../src/render_primitives/render_primitives.rs.html#157))

```rust
156const CUBOID: Cuboid = Cuboid {
157    half_size: Vec3::new(BIG_3D, SMALL_3D, BIG_3D),
158};
159
160const CIRCLE: Circle = Circle { radius: BIG_2D };
161const SPHERE: Sphere = Sphere { radius: BIG_3D };
162
163const ELLIPSE: Ellipse = Ellipse {
164    half_size: Vec2::new(BIG_2D, SMALL_2D),
165};
166
167const TRIANGLE_2D: Triangle2d = Triangle2d {
168    vertices: [
169        Vec2::new(BIG_2D, 0.0),
170        Vec2::new(0.0, BIG_2D),
171        Vec2::new(-BIG_2D, 0.0),
172    ],
173};
174
175const TRIANGLE_3D: Triangle3d = Triangle3d {
176    vertices: [
177        Vec3::new(BIG_3D, 0.0, 0.0),
178        Vec3::new(0.0, BIG_3D, 0.0),
179        Vec3::new(-BIG_3D, 0.0, 0.0),
180    ],
181};
182
183const PLANE_2D: Plane2d = Plane2d { normal: Dir2::Y };
184const PLANE_3D: Plane3d = Plane3d {
185    normal: Dir3::Y,
186    half_size: Vec2::new(BIG_3D, BIG_3D),
187};
188
189const LINE_2D: Line2d = Line2d { direction: Dir2::X };
190const LINE_3D: Line3d = Line3d { direction: Dir3::X };
191
192const SEGMENT_2D: Segment2d = Segment2d {
193    vertices: [Vec2::new(-BIG_2D / 2., 0.), Vec2::new(BIG_2D / 2., 0.)],
194};
195
196const SEGMENT_3D: Segment3d = Segment3d {
197    vertices: [
198        Vec3::new(-BIG_3D / 2., 0., 0.),
199        Vec3::new(BIG_3D / 2., 0., 0.),
200    ],
201};
202
203const POLYLINE_2D_VERTICES: [Vec2; 4] = [
204    Vec2::new(-BIG_2D, -SMALL_2D),
205    Vec2::new(-SMALL_2D, SMALL_2D),
206    Vec2::new(SMALL_2D, -SMALL_2D),
207    Vec2::new(BIG_2D, SMALL_2D),
208];
209
210const POLYLINE_3D_VERTICES: [Vec3; 4] = [
211    Vec3::new(-BIG_3D, -SMALL_3D, -SMALL_3D),
212    Vec3::new(SMALL_3D, SMALL_3D, 0.0),
213    Vec3::new(-SMALL_3D, -SMALL_3D, 0.0),
214    Vec3::new(BIG_3D, SMALL_3D, SMALL_3D),
215];
216
217const CONVEX_POLYGON_VERTICES: [Vec2; 5] = [
218    Vec2::new(-BIG_2D, -SMALL_2D),
219    Vec2::new(BIG_2D, -SMALL_2D),
220    Vec2::new(BIG_2D, SMALL_2D),
221    Vec2::new(BIG_2D / 2.0, SMALL_2D * 2.0),
222    Vec2::new(-BIG_2D, SMALL_2D),
223];
224
225const REGULAR_POLYGON: RegularPolygon = RegularPolygon {
226    circumcircle: Circle { radius: BIG_2D },
227    sides: 5,
228};
229
230const CAPSULE_2D: Capsule2d = Capsule2d {
231    radius: SMALL_2D,
232    half_length: SMALL_2D,
233};
234
235const CAPSULE_3D: Capsule3d = Capsule3d {
236    radius: SMALL_3D,
237    half_length: SMALL_3D,
238};
239
240const CYLINDER: Cylinder = Cylinder {
241    radius: SMALL_3D,
242    half_height: SMALL_3D,
243};
244
245const CONE: Cone = Cone {
246    radius: BIG_3D,
247    height: BIG_3D,
248};
249
250const CONICAL_FRUSTUM: ConicalFrustum = ConicalFrustum {
251    radius_top: BIG_3D,
252    radius_bottom: SMALL_3D,
253    height: BIG_3D,
254};
255
256const ANNULUS: Annulus = Annulus {
257    inner_circle: Circle { radius: SMALL_2D },
258    outer_circle: Circle { radius: BIG_2D },
259};
260
261const TORUS: Torus = Torus {
262    minor_radius: SMALL_3D / 2.0,
263    major_radius: SMALL_3D * 1.5,
264};
265
266const TETRAHEDRON: Tetrahedron = Tetrahedron {
267    vertices: [
268        Vec3::new(-BIG_3D, 0.0, 0.0),
269        Vec3::new(BIG_3D, 0.0, 0.0),
270        Vec3::new(0.0, 0.0, -BIG_3D * 1.67),
271        Vec3::new(0.0, BIG_3D * 1.67, -BIG_3D * 0.5),
272    ],
273};
274
275const ARC: Arc2d = Arc2d {
276    radius: BIG_2D,
277    half_angle: std::f32::consts::FRAC_PI_4,
278};
279
280const CIRCULAR_SECTOR: CircularSector = CircularSector {
281    arc: Arc2d {
282        radius: BIG_2D,
283        half_angle: std::f32::consts::FRAC_PI_4,
284    },
285};
286
287const CIRCULAR_SEGMENT: CircularSegment = CircularSegment {
288    arc: Arc2d {
289        radius: BIG_2D,
290        half_angle: std::f32::consts::FRAC_PI_4,
291    },
292};
293
294fn setup_cameras(mut commands: Commands) {
295    let start_in_2d = true;
296    let make_camera = |is_active| Camera {
297        is_active,
298        ..Default::default()
299    };
300
301    commands.spawn((Camera2d, make_camera(start_in_2d)));
302
303    commands.spawn((
304        Camera3d::default(),
305        make_camera(!start_in_2d),
306        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
307    ));
308}
309
310fn setup_ambient_light(mut ambient_light: ResMut<GlobalAmbientLight>) {
311    ambient_light.brightness = 50.0;
312}
313
314fn setup_lights(mut commands: Commands) {
315    commands.spawn((
316        PointLight {
317            intensity: 5000.0,
318            ..default()
319        },
320        Transform::from_translation(Vec3::new(-LEFT_RIGHT_OFFSET_3D, 2.0, 0.0))
321            .looking_at(Vec3::new(-LEFT_RIGHT_OFFSET_3D, 0.0, 0.0), Vec3::Y),
322    ));
323}
324
325/// Marker component for header text
326#[derive(Debug, Clone, Component, Default, Reflect)]
327pub struct HeaderText;
328
329/// Marker component for header node
330#[derive(Debug, Clone, Component, Default, Reflect)]
331pub struct HeaderNode;
332
333fn update_active_cameras(
334    state: Res<State<CameraActive>>,
335    camera_2d: Single<(Entity, &mut Camera), With<Camera2d>>,
336    camera_3d: Single<(Entity, &mut Camera), (With<Camera3d>, Without<Camera2d>)>,
337    mut text: Query<&mut UiTargetCamera, With<HeaderNode>>,
338) {
339    let (entity_2d, mut cam_2d) = camera_2d.into_inner();
340    let (entity_3d, mut cam_3d) = camera_3d.into_inner();
341    let is_camera_2d_active = matches!(*state.get(), CameraActive::Dim2);
342
343    cam_2d.is_active = is_camera_2d_active;
344    cam_3d.is_active = !is_camera_2d_active;
345
346    let active_camera = if is_camera_2d_active {
347        entity_2d
348    } else {
349        entity_3d
350    };
351
352    text.iter_mut().for_each(|mut target_camera| {
353        *target_camera = UiTargetCamera(active_camera);
354    });
355}
356
357fn switch_cameras(current: Res<State<CameraActive>>, mut next: ResMut<NextState<CameraActive>>) {
358    let next_state = match current.get() {
359        CameraActive::Dim2 => CameraActive::Dim3,
360        CameraActive::Dim3 => CameraActive::Dim2,
361    };
362    next.set(next_state);
363}
364
365fn setup_text(mut commands: Commands, cameras: Query<(Entity, &Camera)>) {
366    let active_camera = cameras
367        .iter()
368        .find_map(|(entity, camera)| camera.is_active.then_some(entity))
369        .expect("run condition ensures existence");
370    commands.spawn((
371        HeaderNode,
372        Node {
373            justify_self: JustifySelf::Center,
374            top: px(5),
375            ..Default::default()
376        },
377        UiTargetCamera(active_camera),
378        children![(
379            Text::default(),
380            HeaderText,
381            TextLayout::justify(Justify::Center),
382            children![
383                TextSpan::new("Primitive: "),
384                TextSpan(format!("{text}", text = PrimitiveSelected::default())),
385                TextSpan::new("\n\n"),
386                TextSpan::new(
387                    "Press 'C' to switch between 2D and 3D mode\n\
388                    Press 'Up' or 'Down' to switch to the next/previous primitive",
389                ),
390                TextSpan::new("\n\n"),
391                TextSpan::new("(If nothing is displayed, there's no rendering support yet)",),
392            ]
393        )],
394    ));
395}
396
397fn update_text(
398    primitive_state: Res<State<PrimitiveSelected>>,
399    header: Query<Entity, With<HeaderText>>,
400    mut writer: TextUiWriter,
401) {
402    let new_text = format!("{text}", text = primitive_state.get());
403    header.iter().for_each(|header_text| {
404        if let Some(mut text) = writer.get_text(header_text, 2) {
405            (*text).clone_from(&new_text);
406        };
407    });
408}
409
410fn switch_to_next_primitive(
411    current: Res<State<PrimitiveSelected>>,
412    mut next: ResMut<NextState<PrimitiveSelected>>,
413) {
414    let next_state = current.get().next();
415    next.set(next_state);
416}
417
418fn switch_to_previous_primitive(
419    current: Res<State<PrimitiveSelected>>,
420    mut next: ResMut<NextState<PrimitiveSelected>>,
421) {
422    let next_state = current.get().previous();
423    next.set(next_state);
424}
425
426fn in_mode(active: CameraActive) -> impl Fn(Res<State<CameraActive>>) -> bool {
427    move |state| *state.get() == active
428}
429
430fn draw_gizmos_2d(mut gizmos: Gizmos, state: Res<State<PrimitiveSelected>>, time: Res<Time>) {
431    const POSITION: Vec2 = Vec2::new(-LEFT_RIGHT_OFFSET_2D, 0.0);
432    let angle = time.elapsed_secs();
433    let isometry = Isometry2d::new(POSITION, Rot2::radians(angle));
434    let color = Color::WHITE;
435
436    #[expect(
437        clippy::match_same_arms,
438        reason = "Certain primitives don't have any 2D rendering support yet."
439    )]
440    match state.get() {
441        PrimitiveSelected::RectangleAndCuboid => {
442            gizmos.primitive_2d(&RECTANGLE, isometry, color);
443        }
444        PrimitiveSelected::CircleAndSphere => {
445            gizmos.primitive_2d(&CIRCLE, isometry, color);
446        }
447        PrimitiveSelected::Ellipse => drop(gizmos.primitive_2d(&ELLIPSE, isometry, color)),
448        PrimitiveSelected::Triangle => gizmos.primitive_2d(&TRIANGLE_2D, isometry, color),
449        PrimitiveSelected::Plane => gizmos.primitive_2d(&PLANE_2D, isometry, color),
450        PrimitiveSelected::Line => drop(gizmos.primitive_2d(&LINE_2D, isometry, color)),
451        PrimitiveSelected::Segment => {
452            drop(gizmos.primitive_2d(&SEGMENT_2D, isometry, color));
453        }
454        PrimitiveSelected::Polyline => gizmos.primitive_2d(
455            &Polyline2d {
456                vertices: POLYLINE_2D_VERTICES.to_vec(),
457            },
458            isometry,
459            color,
460        ),
461        PrimitiveSelected::ConvexPolygon => gizmos.primitive_2d(
462            &Polygon::from(ConvexPolygon::new(CONVEX_POLYGON_VERTICES).unwrap()),
463            isometry,
464            color,
465        ),
466        PrimitiveSelected::Polygon => gizmos.primitive_2d(
467            &Polygon {
468                vertices: vec![
469                    Vec2::new(-BIG_2D, -SMALL_2D),
470                    Vec2::new(BIG_2D, -SMALL_2D),
471                    Vec2::new(BIG_2D, SMALL_2D),
472                    Vec2::new(0.0, 0.0),
473                    Vec2::new(-BIG_2D, SMALL_2D),
474                ],
475            },
476            isometry,
477            color,
478        ),
479        PrimitiveSelected::RegularPolygon => {
480            gizmos.primitive_2d(&REGULAR_POLYGON, isometry, color);
481        }
482        PrimitiveSelected::Capsule => gizmos.primitive_2d(&CAPSULE_2D, isometry, color),
483        PrimitiveSelected::Cylinder => {}
484        PrimitiveSelected::Cone => {}
485        PrimitiveSelected::ConicalFrustum => {}
486        PrimitiveSelected::Torus => drop(gizmos.primitive_2d(&ANNULUS, isometry, color)),
487        PrimitiveSelected::Tetrahedron => {}
488        PrimitiveSelected::Arc => gizmos.primitive_2d(&ARC, isometry, color),
489        PrimitiveSelected::CircularSector => {
490            gizmos.primitive_2d(&CIRCULAR_SECTOR, isometry, color);
491        }
492        PrimitiveSelected::CircularSegment => {
493            gizmos.primitive_2d(&CIRCULAR_SEGMENT, isometry, color);
494        }
495    }
496}
497
498/// Marker for primitive meshes to record in which state they should be visible in
499#[derive(Debug, Clone, Component, Default, Reflect)]
500pub struct PrimitiveData {
501    camera_mode: CameraActive,
502    primitive_state: PrimitiveSelected,
503}
504
505/// Marker for meshes of 2D primitives
506#[derive(Debug, Clone, Component, Default)]
507pub struct MeshDim2;
508
509/// Marker for meshes of 3D primitives
510#[derive(Debug, Clone, Component, Default)]
511pub struct MeshDim3;
512
513fn spawn_primitive_2d(
514    mut commands: Commands,
515    mut materials: ResMut<Assets<ColorMaterial>>,
516    mut meshes: ResMut<Assets<Mesh>>,
517) {
518    const POSITION: Vec3 = Vec3::new(LEFT_RIGHT_OFFSET_2D, 0.0, 0.0);
519    let material: Handle<ColorMaterial> = materials.add(Color::WHITE);
520    let camera_mode = CameraActive::Dim2;
521    let polyline_2d = Polyline2d {
522        vertices: POLYLINE_2D_VERTICES.to_vec(),
523    };
524    let convex_polygon = ConvexPolygon::new(CONVEX_POLYGON_VERTICES).unwrap();
525    [
526        Some(RECTANGLE.mesh().build()),
527        Some(CIRCLE.mesh().build()),
528        Some(ELLIPSE.mesh().build()),
529        Some(TRIANGLE_2D.mesh().build()),
530        None, // plane
531        None, // line
532        Some(SEGMENT_2D.mesh().build()),
533        Some(polyline_2d.mesh().build()),
534        None, // polygon
535        Some(convex_polygon.mesh().build()),
536        Some(REGULAR_POLYGON.mesh().build()),
537        Some(CAPSULE_2D.mesh().build()),
538        None, // cylinder
539        None, // cone
540        None, // conical frustum
541        Some(ANNULUS.mesh().build()),
542        None, // tetrahedron
543        None, // arc
544        Some(CIRCULAR_SECTOR.mesh().build()),
545        Some(CIRCULAR_SEGMENT.mesh().build()),
546    ]
547    .into_iter()
548    .zip(PrimitiveSelected::ALL)
549    .for_each(|(maybe_mesh, state)| {
550        if let Some(mesh) = maybe_mesh {
551            commands.spawn((
552                MeshDim2,
553                PrimitiveData {
554                    camera_mode,
555                    primitive_state: state,
556                },
557                Mesh2d(meshes.add(mesh)),
558                MeshMaterial2d(material.clone()),
559                Transform::from_translation(POSITION),
560            ));
561        }
562    });
563}
564
565fn spawn_primitive_3d(
566    mut commands: Commands,
567    mut materials: ResMut<Assets<StandardMaterial>>,
568    mut meshes: ResMut<Assets<Mesh>>,
569) {
570    const POSITION: Vec3 = Vec3::new(-LEFT_RIGHT_OFFSET_3D, 0.0, 0.0);
571    let material: Handle<StandardMaterial> = materials.add(Color::WHITE);
572    let camera_mode = CameraActive::Dim3;
573    let polyline_3d = Polyline3d {
574        vertices: POLYLINE_3D_VERTICES.to_vec(),
575    };
576    [
577        Some(CUBOID.mesh().build()),
578        Some(SPHERE.mesh().build()),
579        None, // ellipse
580        Some(TRIANGLE_3D.mesh().build()),
581        Some(PLANE_3D.mesh().build()),
582        None, // line
583        Some(SEGMENT_3D.mesh().build()),
584        Some(polyline_3d.mesh().build()),
585        None, // polygon
586        None, // convex polygon
587        None, // regular polygon
588        Some(CAPSULE_3D.mesh().build()),
589        Some(CYLINDER.mesh().build()),
590        Some(CONE.mesh().build()),
591        Some(CONICAL_FRUSTUM.mesh().build()),
592        Some(TORUS.mesh().build()),
593        Some(TETRAHEDRON.mesh().build()),
594        None, // arc
595        None, // circular sector
596        None, // circular segment
597    ]
598    .into_iter()
599    .zip(PrimitiveSelected::ALL)
600    .for_each(|(maybe_mesh, state)| {
601        if let Some(mesh) = maybe_mesh {
602            commands.spawn((
603                MeshDim3,
604                PrimitiveData {
605                    camera_mode,
606                    primitive_state: state,
607                },
608                Mesh3d(meshes.add(mesh)),
609                MeshMaterial3d(material.clone()),
610                Transform::from_translation(POSITION),
611            ));
612        }
613    });
614}
615
616fn update_primitive_meshes(
617    camera_state: Res<State<CameraActive>>,
618    primitive_state: Res<State<PrimitiveSelected>>,
619    mut primitives: Query<(&mut Visibility, &PrimitiveData)>,
620) {
621    primitives.iter_mut().for_each(|(mut vis, primitive)| {
622        let visible = primitive.camera_mode == *camera_state.get()
623            && primitive.primitive_state == *primitive_state.get();
624        *vis = if visible {
625            Visibility::Inherited
626        } else {
627            Visibility::Hidden
628        };
629    });
630}
631
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
647
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

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 53](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#53))

```rust
50fn setup(mut commands: Commands) {
51    commands.spawn((
52        Camera3d::default(),
53        Transform::from_xyz(0.0, 7.5, 18.0).looking_at(Vec3::new(0.0, 5.5, 0.0), Vec3::Y),
54    ));
55}
56
57#[derive(Component, Debug, Default)]
58struct PendingScene(Handle<Gltf>);
59
60#[derive(Component, Debug, Default)]
61struct PendingAnimation((Handle<AnimationGraph>, AnimationNodeIndex));
62
63fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
64    commands.spawn((
65        PendingScene(asset_server.load("models/animated/Fox.glb")),
66        Transform::from_xyz(1.3, 4.3, 0.0)
67            .with_scale(Vec3::splat(0.08))
68            .looking_to(-Vec3::X, Vec3::Y),
69    ));
70}
71
72fn spawn_scene(
73    mut commands: Commands,
74    query: Query<(Entity, &PendingScene)>,
75    assets: Res<Assets<Gltf>>,
76    mut graphs: ResMut<Assets<AnimationGraph>>,
77) {
78    for (entity, PendingScene(asset)) in query.iter() {
79        if let Some(gltf) = assets.get(asset)
80            && let Some(scene_handle) = gltf.scenes.first()
81            && let Some(animation_handle) = gltf.named_animations.get("Run")
82        {
83            let (graph, graph_node_index) = AnimationGraph::from_clip(animation_handle.clone());
84
85            commands
86                .entity(entity)
87                .remove::<PendingScene>()
88                .insert((
89                    WorldAssetRoot(scene_handle.clone()),
90                    PendingAnimation((graphs.add(graph), graph_node_index)),
91                ))
92                .observe(play_animation);
93        }
94    }
95}
96
97fn play_animation(
98    trigger: On<WorldInstanceReady>,
99    mut commands: Commands,
100    children: Query<&Children>,
101    animations: Query<&PendingAnimation>,
102    mut players: Query<&mut AnimationPlayer>,
103) {
104    if let Ok(PendingAnimation((graph_handle, graph_node_index))) = animations.get(trigger.entity) {
105        for child in children.iter_descendants(trigger.entity) {
106            if let Ok(mut player) = players.get_mut(child) {
107                player.play(*graph_node_index).set_speed(0.6).repeat();
108
109                commands
110                    .entity(child)
111                    .insert(AnimationGraphHandle(graph_handle.clone()));
112            }
113        }
114    }
115
116    commands.entity(trigger.entity).remove::<PendingAnimation>();
117}
118
119type CustomAnimationId = i8;
120
121#[derive(Component)]
122struct CustomAnimation(CustomAnimationId);
123
124fn spawn_custom_meshes(
125    mut commands: Commands,
126    mut mesh_assets: ResMut<Assets<Mesh>>,
127    mut material_assets: ResMut<Assets<StandardMaterial>>,
128    mut inverse_bindposes_assets: ResMut<Assets<SkinnedMeshInverseBindposes>>,
129) {
130    let mesh_handle = mesh_assets.add(
131        Mesh::new(
132            PrimitiveTopology::TriangleStrip,
133            // Test that skinned mesh bounds work even if the mesh is render
134            // world only.
135            RenderAssetUsages::RENDER_WORLD,
136        )
137        .with_inserted_attribute(
138            Mesh::ATTRIBUTE_POSITION,
139            vec![
140                [-0.5, 0.0, 0.0],
141                [0.5, 0.0, 0.0],
142                [-0.5, 0.5, 0.0],
143                [0.5, 0.5, 0.0],
144                [-0.5, 1.0, 0.0],
145                [0.5, 1.0, 0.0],
146                [-0.5, 1.5, 0.0],
147                [0.5, 1.5, 0.0],
148                [-0.5, 2.0, 0.0],
149                [0.5, 2.0, 0.0],
150            ],
151        )
152        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 10])
153        .with_inserted_attribute(
154            Mesh::ATTRIBUTE_JOINT_INDEX,
155            VertexAttributeValues::Uint16x4(vec![
156                [1, 0, 0, 0],
157                [1, 0, 0, 0],
158                [1, 2, 0, 0],
159                [1, 2, 0, 0],
160                [1, 2, 0, 0],
161                [1, 2, 0, 0],
162                [2, 1, 0, 0],
163                [2, 1, 0, 0],
164                [2, 0, 0, 0],
165                [2, 0, 0, 0],
166            ]),
167        )
168        .with_inserted_attribute(
169            Mesh::ATTRIBUTE_JOINT_WEIGHT,
170            vec![
171                [1.00, 0.00, 0.0, 0.0],
172                [1.00, 0.00, 0.0, 0.0],
173                [0.75, 0.25, 0.0, 0.0],
174                [0.75, 0.25, 0.0, 0.0],
175                [0.50, 0.50, 0.0, 0.0],
176                [0.50, 0.50, 0.0, 0.0],
177                [0.75, 0.25, 0.0, 0.0],
178                [0.75, 0.25, 0.0, 0.0],
179                [1.00, 0.00, 0.0, 0.0],
180                [1.00, 0.00, 0.0, 0.0],
181            ],
182        )
183        .with_generated_skinned_mesh_bounds()
184        .unwrap(),
185    );
186
187    let inverse_bindposes_handle = inverse_bindposes_assets.add(vec![
188        Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)),
189        Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)),
190        Mat4::from_translation(Vec3::new(0.0, -1.0, 0.0)),
191    ]);
192
193    struct MeshInstance {
194        animations: [CustomAnimationId; 2],
195    }
196
197    let mesh_instances = [
198        // Simple cases. First joint is still, second joint is all rotation/translation/scale variations.
199        MeshInstance { animations: [0, 1] },
200        MeshInstance { animations: [0, 2] },
201        MeshInstance { animations: [0, 3] },
202        MeshInstance { animations: [0, 4] },
203        MeshInstance { animations: [0, 5] },
204        MeshInstance { animations: [0, 6] },
205        MeshInstance { animations: [0, 7] },
206        MeshInstance { animations: [0, 8] },
207        // Skewed cases. First joint is non-uniform scaling, second joint is rotation/translation variations.
208        MeshInstance { animations: [9, 1] },
209        MeshInstance { animations: [9, 2] },
210        MeshInstance { animations: [9, 3] },
211        MeshInstance { animations: [9, 4] },
212        MeshInstance { animations: [9, 5] },
213    ];
214
215    for (i, mesh_instance) in mesh_instances.iter().enumerate() {
216        let x = ((i as f32) * 2.0) - ((mesh_instances.len() - 1) as f32);
217
218        let base_entity = commands
219            .spawn((Transform::from_xyz(x, 0.0, 0.0), Visibility::default()))
220            .id();
221
222        let joints = vec![
223            commands.spawn((Transform::IDENTITY,)).id(),
224            commands
225                .spawn((
226                    CustomAnimation(mesh_instance.animations[0]),
227                    Transform::IDENTITY,
228                ))
229                .id(),
230            commands
231                .spawn((
232                    CustomAnimation(mesh_instance.animations[1]),
233                    Transform::IDENTITY,
234                ))
235                .id(),
236        ];
237
238        commands.entity(joints[0]).insert(ChildOf(base_entity));
239
240        commands.entity(joints[1]).insert(ChildOf(joints[0]));
241        commands.entity(joints[2]).insert(ChildOf(joints[1]));
242
243        let mesh_entity = commands
244            .spawn((
245                Transform::IDENTITY,
246                Mesh3d(mesh_handle.clone()),
247                MeshMaterial3d(material_assets.add(StandardMaterial {
248                    base_color: Color::WHITE,
249                    cull_mode: None,
250                    ..default()
251                })),
252                SkinnedMesh {
253                    inverse_bindposes: inverse_bindposes_handle.clone(),
254                    joints: joints.clone(),
255                },
256                DynamicSkinnedMeshBounds,
257            ))
258            .id();
259
260        commands.entity(mesh_entity).insert(ChildOf(base_entity));
261    }
262}
263
264fn update_custom_mesh_animation(
265    time: Res<Time<Virtual>>,
266    mut query: Query<(&mut Transform, &CustomAnimation)>,
267) {
268    let t = time.elapsed_secs();
269    let ts = ops::sin(t);
270    let tc = ops::cos(t);
271    let ots = ops::sin(t + FRAC_PI_4);
272    let otc = ops::cos(t + FRAC_PI_4);
273
274    for (mut transform, animation) in &mut query {
275        match animation.0 {
276            1 => transform.translation = Vec3::new(0.5 * ts, 0.3 + tc, 0.0),
277            2 => transform.translation = Vec3::new(0.0, 0.5 + ts, tc),
278            3 => transform.rotation = Quat::from_rotation_x(FRAC_PI_2 * ts),
279            4 => transform.rotation = Quat::from_rotation_y(FRAC_PI_2 * ts),
280            5 => transform.rotation = Quat::from_rotation_z(FRAC_PI_2 * ts),
281            6 => transform.scale.x = ts * 1.5,
282            7 => transform.scale.y = ts * 1.5,
283            8 => transform.scale = Vec3::new(ts * 1.5, otc * 1.5, 1.0),
284            9 => transform.scale = Vec3::new(ots, 1.0 + (tc * 0.3), 1.0 - (tc * 0.5)),
285            _ => (),
286        }
287    }
288}
```

examples/3d/pccm.rs ([line 98](../../../src/pccm/pccm.rs.html#98))

```rust
94fn spawn_camera(commands: &mut Commands) {
95    commands.spawn((
96        Camera3d::default(),
97        FreeCamera::default(),
98        Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::new(0.0, -2.5, 0.0), Dir3::Y),
99        Hdr,
100    ));
101}
102
103/// Spawns the inner reflective cube in the scene.
104fn spawn_inner_cube(
105    commands: &mut Commands,
106    meshes: &mut Assets<Mesh>,
107    materials: &mut Assets<StandardMaterial>,
108) {
109    let cube_mesh = meshes.add(
110        Cuboid {
111            half_size: Vec3::new(5.0, 1.0, 2.0),
112        }
113        .mesh()
114        .build()
115        .with_duplicated_vertices()
116        .with_computed_flat_normals(),
117    );
118    let cube_material = materials.add(StandardMaterial {
119        base_color: Color::WHITE,
120        metallic: 1.0,
121        reflectance: 1.0,
122        perceptual_roughness: 0.0,
123        ..default()
124    });
125
126    commands.spawn((
127        Mesh3d(cube_mesh),
128        MeshMaterial3d(cube_material),
129        Transform::from_xyz(0.0, -4.0, -2.5),
130        InnerCube,
131    ));
132}
```

examples/gizmos/3d\_text\_gizmos.rs ([line 17](../../../src/3d_text_gizmos/3d_text_gizmos.rs.html#17))

```rust
14fn setup_camera(mut commands: Commands, mut gizmo_config_store: ResMut<GizmoConfigStore>) {
15    commands.spawn((
16        Camera3d::default(),
17        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
18    ));
19
20    let (config, _) = gizmo_config_store.config_mut::<DefaultGizmoConfigGroup>();
21
22    config.line.width = 4.;
23}
24
25fn hello_world(mut text_gizmos: Gizmos, time: Res<Time>) {
26    let t = 0.2 * time.elapsed_secs();
27
28    text_gizmos.text(
29        Isometry3d::new(Vec3::new(0.0, 1.5, 0.0), Quat::from_rotation_y(-t)),
30        "Hello",
31        1.,
32        Vec2::ZERO,
33        RED,
34    );
35
36    text_gizmos.text(
37        Isometry3d::new(Vec3::new(0.0, 0.0, 0.0), Quat::from_rotation_y(t + 0.25)),
38        "Text",
39        1.,
40        Vec2::ZERO,
41        ORANGE,
42    );
43
44    text_gizmos.text(
45        Isometry3d::new(Vec3::new(0.0, -1.5, 0.0), Quat::from_rotation_y(-t - 0.5)),
46        "Gizmos",
47        1.,
48        Vec2::ZERO,
49        YELLOW,
50    );
51}
```

Additional examples can be found in:  

*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../../src/external_source_external_thread/external_source_external_thread.rs.html#69)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../../src/async_channel_pattern/async_channel_pattern.rs.html#167)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#368)
*   [examples/scene/world\_serialization.rs](../../../src/world_serialization/world_serialization.rs.html#129)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#24)
*   [examples/async\_tasks/async\_compute.rs](../../../src/async_compute/async_compute.rs.html#159)
*   [examples/ui/widgets/feathers\_gallery.rs](../../../src/feathers_gallery/feathers_gallery.rs.html#72)
*   [examples/gltf/query\_gltf\_primitives.rs](../../../src/query_gltf_primitives/query_gltf_primitives.rs.html#57)
*   [examples/stress\_tests/bevymark\_3d.rs](../../../src/bevymark_3d/bevymark_3d.rs.html#337)
*   [examples/camera/2d\_top\_down\_camera.rs](../../../src/2d_top_down_camera/2d_top_down_camera.rs.html#74)
*   [examples/shader/fallback\_image.rs](../../../src/fallback_image/fallback_image.rs.html#43)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../../src/fullscreen_material/fullscreen_material.rs.html#33)
*   [examples/3d/parallax\_mapping.rs](../../../src/parallax_mapping/parallax_mapping.rs.html#166)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../../src/texture_binding_array/texture_binding_array.rs.html#61)
*   [examples/2d/sprite\_slice.rs](../../../src/sprite_slice/sprite_slice.rs.html#122)
*   [examples/asset/multi\_asset\_sync.rs](../../../src/multi_asset_sync/multi_asset_sync.rs.html#196)
*   [examples/3d/color\_grading.rs](../../../src/color_grading/color_grading.rs.html#326)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#372)
*   [examples/3d/mesh\_ray\_cast.rs](../../../src/mesh_ray_cast/mesh_ray_cast.rs.html#32)
*   [examples/3d/atmospheric\_fog.rs](../../../src/atmospheric_fog/atmospheric_fog.rs.html#29)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#174)
*   [examples/animation/animated\_mesh.rs](../../../src/animated_mesh/animated_mesh.rs.html#109)
*   [examples/showcase/loading\_screen.rs](../../../src/loading_screen/loading_screen.rs.html#140)
*   [examples/animation/animation\_graph.rs](../../../src/animation_graph/animation_graph.rs.html#229)
*   [examples/3d/post\_processing.rs](../../../src/post_processing/post_processing.rs.html#83)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#24)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#128)
*   [examples/3d/animated\_material.rs](../../../src/animated_material/animated_material.rs.html#21)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../../src/custom_post_processing/custom_post_processing.rs.html#250)
*   [examples/animation/animated\_mesh\_events.rs](../../../src/animated_mesh_events/animated_mesh_events.rs.html#66)
*   [examples/picking/custom\_hit\_data.rs](../../../src/custom_hit_data/custom_hit_data.rs.html#99)
*   [examples/gltf/load\_gltf.rs](../../../src/load_gltf/load_gltf.rs.html#21)
*   [examples/stress\_tests/many\_materials.rs](../../../src/many_materials/many_materials.rs.html#63)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#97)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#202-206)
*   [examples/3d/tonemapping.rs](../../../src/tonemapping/tonemapping.rs.html#34)
*   [examples/gltf/update\_gltf\_scene.rs](../../../src/update_gltf_scene/update_gltf_scene.rs.html#28)
*   [examples/3d/lines.rs](../../../src/lines/lines.rs.html#31)
*   [examples/2d/mesh2d\_vertex\_color\_texture.rs](../../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#41)
*   [examples/shader/array\_texture.rs](../../../src/array_texture/array_texture.rs.html#57)
*   [examples/3d/order\_independent\_transparency.rs](../../../src/order_independent_transparency/order_independent_transparency.rs.html#139)
*   [examples/audio/spatial\_audio\_2d.rs](../../../src/spatial_audio_2d/spatial_audio_2d.rs.html#39)
*   [examples/3d/atmosphere.rs](../../../src/atmosphere/atmosphere.rs.html#237)
*   [examples/animation/animated\_mesh\_control.rs](../../../src/animated_mesh_control/animated_mesh_control.rs.html#54)
*   [examples/asset/generated\_assets.rs](../../../src/generated_assets/generated_assets.rs.html#23)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../../src/custom_shader_instancing/custom_shader_instancing.rs.html#63)
*   [examples/showcase/contributors.rs](../../../src/contributors/contributors.rs.html#105)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#174)
*   [examples/2d/bloom\_2d.rs](../../../src/bloom_2d/bloom_2d.rs.html#47)
*   [examples/gizmos/axes.rs](../../../src/axes/axes.rs.html#61)
*   [examples/camera/free\_camera\_controller.rs](../../../src/free_camera_controller/free_camera_controller.rs.html#287)
*   [examples/3d/motion\_blur.rs](../../../src/motion_blur/motion_blur.rs.html#72)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#243)
*   [examples/animation/eased\_motion.rs](../../../src/eased_motion/eased_motion.rs.html#74)
*   [examples/3d/fog.rs](../../../src/fog/fog.rs.html#87)
*   [examples/animation/easing\_functions.rs](../../../src/easing_functions/easing_functions.rs.html#172)
*   [examples/2d/sprite\_animation.rs](../../../src/sprite_animation/sprite_animation.rs.html#123)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#51)
*   [examples/time/virtual\_time.rs](../../../src/virtual_time/virtual_time.rs.html#70)
*   [examples/3d/shadow\_caster\_receiver.rs](../../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#103)
*   [examples/stress\_tests/many\_lights.rs](../../../src/many_lights/many_lights.rs.html#108)
*   [examples/3d/anti\_aliasing.rs](../../../src/anti_aliasing/anti_aliasing.rs.html#464)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#174)
*   [examples/2d/mesh2d\_arcs.rs](../../../src/mesh2d_arcs/mesh2d_arcs.rs.html#71)
*   [examples/3d/scrolling\_fog.rs](../../../src/scrolling_fog/scrolling_fog.rs.html#50)
*   [examples/ecs/iter\_combinations.rs](../../../src/iter_combinations/iter_combinations.rs.html#56-60)
*   [examples/3d/render\_to\_texture.rs](../../../src/render_to_texture/render_to_texture.rs.html#55)
*   [examples/showcase/alien\_cake\_addict.rs](../../../src/alien_cake_addict/alien_cake_addict.rs.html#159-163)
*   [examples/3d/spotlight.rs](../../../src/spotlight/spotlight.rs.html#102)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#140)
*   [examples/3d/meshlet.rs](../../../src/meshlet/meshlet.rs.html#42)
*   [examples/shader/shader\_prepass.rs](../../../src/shader_prepass/shader_prepass.rs.html#75)
*   [examples/3d/shadow\_biases.rs](../../../src/shadow_biases/shadow_biases.rs.html#70)
*   [examples/3d/split\_screen.rs](../../../src/split_screen/split_screen.rs.html#61)
*   [examples/picking/mesh\_picking.rs](../../../src/mesh_picking/mesh_picking.rs.html#144)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#103)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#159)
*   [examples/2d/texture\_atlas.rs](../../../src/texture_atlas/texture_atlas.rs.html#104)
*   [examples/2d/sprite\_scale.rs](../../../src/sprite_scale/sprite_scale.rs.html#25)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#198-202)
*   [examples/3d/deferred\_rendering.rs](../../../src/deferred_rendering/deferred_rendering.rs.html#37)
*   [examples/3d/contact\_shadows.rs](../../../src/contact_shadows/contact_shadows.rs.html#112)
*   [examples/2d/text2d.rs](../../../src/text2d/text2d.rs.html#68)
*   [examples/3d/3d\_shapes.rs](../../../src/3d_shapes/3d_shapes.rs.html#80)
*   [examples/animation/custom\_skinned\_mesh.rs](../../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#53)
*   [examples/animation/animated\_transform.rs](../../../src/animated_transform/animated_transform.rs.html#59)
*   [examples/3d/lighting.rs](../../../src/lighting/lighting.rs.html#158)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#141)
*   [examples/stress\_tests/many\_cubes.rs](../../../src/many_cubes/many_cubes.rs.html#306-310)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#103)

#### pub const fn [splat](#method.splat)(v: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Creates a vector with all elements set to `v`.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/stress\_tests/bevymark\_3d.rs ([line 28](../../../src/bevymark_3d/bevymark_3d.rs.html#28))

```rust
28const VOLUME_SIZE: Vec3 = Vec3::splat(VOLUME_WIDTH as f32);
29
30#[derive(Resource)]
31struct BevyCounter {
32    pub count: usize,
33    pub color: Color,
34}
35
36#[derive(Component)]
37struct Cube {
38    velocity: Vec3,
39}
40
41#[derive(FromArgs, Resource)]
42/// `bevymark_3d` cube stress test
43struct Args {
44    /// whether to step animations by a fixed amount such that each frame is the same across runs.
45    /// If spawning waves, all are spawned up-front to immediately start rendering at the heaviest
46    /// load.
47    #[argh(switch)]
48    benchmark: bool,
49
50    /// how many cubes to spawn per wave.
51    #[argh(option, default = "0")]
52    per_wave: usize,
53
54    /// the number of waves to spawn.
55    #[argh(option, default = "0")]
56    waves: usize,
57
58    /// whether to vary the material data in each instance.
59    #[argh(switch)]
60    vary_per_instance: bool,
61
62    /// the number of different textures from which to randomly select the material color. 0 means no textures.
63    #[argh(option, default = "1")]
64    material_texture_count: usize,
65
66    /// the alpha mode used to spawn the cubes
67    #[argh(option, default = "AlphaMode::Opaque")]
68    alpha_mode: AlphaMode,
69}
70
71#[derive(Default, Clone)]
72enum AlphaMode {
73    #[default]
74    Opaque,
75    Blend,
76    AlphaMask,
77}
78
79impl FromStr for AlphaMode {
80    type Err = String;
81
82    fn from_str(s: &str) -> Result<Self, Self::Err> {
83        match s {
84            "opaque" => Ok(Self::Opaque),
85            "blend" => Ok(Self::Blend),
86            "alpha_mask" => Ok(Self::AlphaMask),
87            _ => Err(format!(
88                "Unknown alpha mode: '{s}', valid modes: 'opaque', 'blend', 'alpha_mask'"
89            )),
90        }
91    }
92}
93
94const FIXED_TIMESTEP: f32 = 0.2;
95
96fn main() {
97    // `from_env` panics on the web
98    #[cfg(not(target_arch = "wasm32"))]
99    let args: Args = argh::from_env();
100    #[cfg(target_arch = "wasm32")]
101    let args = Args::from_args(&[], &[]).unwrap();
102
103    App::new()
104        .add_plugins((
105            DefaultPlugins.set(WindowPlugin {
106                primary_window: Some(Window {
107                    title: "BevyMark 3D".into(),
108                    resolution: WindowResolution::new(1920, 1080).with_scale_factor_override(1.0),
109                    present_mode: PresentMode::AutoNoVsync,
110                    ..default()
111                }),
112                ..default()
113            }),
114            FrameTimeDiagnosticsPlugin::default(),
115            LogDiagnosticsPlugin::default(),
116        ))
117        .insert_resource(WinitSettings::continuous())
118        .insert_resource(args)
119        .insert_resource(BevyCounter {
120            count: 0,
121            color: Color::WHITE,
122        })
123        .add_systems(Startup, setup)
124        .add_systems(FixedUpdate, scheduled_spawner)
125        .add_systems(
126            Update,
127            (
128                mouse_handler,
129                movement_system,
130                collision_system,
131                counter_system,
132            ),
133        )
134        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
135            FIXED_TIMESTEP,
136        )))
137        .run();
138}
139
140#[derive(Resource)]
141struct CubeScheduled {
142    waves: usize,
143    per_wave: usize,
144}
145
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
```

Hide additional examples

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 67](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#67))

```rust
63fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
64    commands.spawn((
65        PendingScene(asset_server.load("models/animated/Fox.glb")),
66        Transform::from_xyz(1.3, 4.3, 0.0)
67            .with_scale(Vec3::splat(0.08))
68            .looking_to(-Vec3::X, Vec3::Y),
69    ));
70}
```

examples/3d/irradiance\_volumes.rs ([line 273](../../../src/irradiance_volumes/irradiance_volumes.rs.html#273))

```rust
268fn spawn_sphere(commands: &mut Commands, assets: &ExampleAssets) {
269    commands
270        .spawn((
271            Mesh3d(assets.main_sphere.clone()),
272            MeshMaterial3d(assets.main_sphere_material.clone()),
273            Transform::from_xyz(0.0, SPHERE_SCALE, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
274        ))
275        .insert(MainObject);
276}
277
278fn spawn_voxel_cube_parent(commands: &mut Commands) {
279    commands.spawn((Visibility::Hidden, Transform::default(), VoxelCubeParent));
280}
281
282fn spawn_fox(commands: &mut Commands, assets: &ExampleAssets) {
283    commands.spawn((
284        WorldAssetRoot(assets.fox.clone()),
285        Visibility::Hidden,
286        Transform::from_scale(Vec3::splat(FOX_SCALE)),
287        MainObject,
288    ));
289}
290
291fn spawn_text(commands: &mut Commands, app_status: &AppStatus) {
292    commands.spawn((
293        app_status.create_text(),
294        Node {
295            position_type: PositionType::Absolute,
296            bottom: px(12),
297            left: px(12),
298            ..default()
299        },
300    ));
301}
302
303// A system that updates the help text.
304fn update_text(mut text_query: Query<&mut Text>, app_status: Res<AppStatus>) {
305    for mut text in text_query.iter_mut() {
306        *text = app_status.create_text();
307    }
308}
309
310impl AppStatus {
311    // Constructs the help text at the bottom of the screen based on the
312    // application status.
313    fn create_text(&self) -> Text {
314        let irradiance_volume_help_text = if self.irradiance_volume_present {
315            DISABLE_IRRADIANCE_VOLUME_HELP_TEXT
316        } else {
317            ENABLE_IRRADIANCE_VOLUME_HELP_TEXT
318        };
319
320        let voxels_help_text = if self.voxels_visible {
321            HIDE_VOXELS_HELP_TEXT
322        } else {
323            SHOW_VOXELS_HELP_TEXT
324        };
325
326        let rotation_help_text = if self.rotating {
327            STOP_ROTATION_HELP_TEXT
328        } else {
329            START_ROTATION_HELP_TEXT
330        };
331
332        let switch_mesh_help_text = match self.model {
333            ExampleModel::Sphere => SWITCH_TO_FOX_HELP_TEXT,
334            ExampleModel::Fox => SWITCH_TO_SPHERE_HELP_TEXT,
335        };
336
337        format!(
338            "{CLICK_TO_MOVE_HELP_TEXT}\n\
339            {voxels_help_text}\n\
340            {irradiance_volume_help_text}\n\
341            {rotation_help_text}\n\
342            {switch_mesh_help_text}"
343        )
344        .into()
345    }
346}
347
348// Rotates the camera a bit every frame.
349fn rotate_camera(
350    mut camera_query: Query<&mut Transform, With<Camera3d>>,
351    time: Res<Time>,
352    app_status: Res<AppStatus>,
353) {
354    if !app_status.rotating {
355        return;
356    }
357
358    for mut transform in camera_query.iter_mut() {
359        transform.translation = Vec2::from_angle(ROTATION_SPEED * time.delta_secs())
360            .rotate(transform.translation.xz())
361            .extend(transform.translation.y)
362            .xzy();
363        transform.look_at(Vec3::ZERO, Vec3::Y);
364    }
365}
366
367// Toggles between the unskinned sphere model and the skinned fox model if the
368// user requests it.
369fn change_main_object(
370    keyboard: Res<ButtonInput<KeyCode>>,
371    mut app_status: ResMut<AppStatus>,
372    mut sphere_query: Query<
373        &mut Visibility,
374        (With<MainObject>, With<Mesh3d>, Without<WorldAssetRoot>),
375    >,
376    mut fox_query: Query<&mut Visibility, (With<MainObject>, With<WorldAssetRoot>)>,
377) {
378    if !keyboard.just_pressed(KeyCode::Tab) {
379        return;
380    }
381    let Some(mut sphere_visibility) = sphere_query.iter_mut().next() else {
382        return;
383    };
384    let Some(mut fox_visibility) = fox_query.iter_mut().next() else {
385        return;
386    };
387
388    match app_status.model {
389        ExampleModel::Sphere => {
390            *sphere_visibility = Visibility::Hidden;
391            *fox_visibility = Visibility::Visible;
392            app_status.model = ExampleModel::Fox;
393        }
394        ExampleModel::Fox => {
395            *sphere_visibility = Visibility::Visible;
396            *fox_visibility = Visibility::Hidden;
397            app_status.model = ExampleModel::Sphere;
398        }
399    }
400}
401
402impl Default for AppStatus {
403    fn default() -> Self {
404        Self {
405            irradiance_volume_present: true,
406            rotating: true,
407            model: ExampleModel::Sphere,
408            voxels_visible: false,
409        }
410    }
411}
412
413// Turns on and off the irradiance volume as requested by the user.
414fn toggle_irradiance_volumes(
415    mut commands: Commands,
416    keyboard: Res<ButtonInput<KeyCode>>,
417    light_probe_query: Query<Entity, With<LightProbe>>,
418    mut app_status: ResMut<AppStatus>,
419    assets: Res<ExampleAssets>,
420    mut ambient_light: ResMut<GlobalAmbientLight>,
421) {
422    if !keyboard.just_pressed(KeyCode::Space) {
423        return;
424    };
425
426    let Some(light_probe) = light_probe_query.iter().next() else {
427        return;
428    };
429
430    if app_status.irradiance_volume_present {
431        commands.entity(light_probe).remove::<IrradianceVolume>();
432        ambient_light.brightness = AMBIENT_LIGHT_BRIGHTNESS * IRRADIANCE_VOLUME_INTENSITY;
433        app_status.irradiance_volume_present = false;
434    } else {
435        commands.entity(light_probe).insert(IrradianceVolume {
436            voxels: assets.irradiance_volume.clone(),
437            intensity: IRRADIANCE_VOLUME_INTENSITY,
438            ..default()
439        });
440        ambient_light.brightness = 0.0;
441        app_status.irradiance_volume_present = true;
442    }
443}
444
445fn toggle_rotation(keyboard: Res<ButtonInput<KeyCode>>, mut app_status: ResMut<AppStatus>) {
446    if keyboard.just_pressed(KeyCode::Enter) {
447        app_status.rotating = !app_status.rotating;
448    }
449}
450
451// Handles clicks on the plane that reposition the object.
452fn handle_mouse_clicks(
453    buttons: Res<ButtonInput<MouseButton>>,
454    windows: Query<&Window, With<PrimaryWindow>>,
455    cameras: Query<(&Camera, &GlobalTransform)>,
456    mut main_objects: Query<&mut Transform, With<MainObject>>,
457) {
458    if !buttons.pressed(MouseButton::Left) {
459        return;
460    }
461    let Some(mouse_position) = windows.iter().next().and_then(Window::cursor_position) else {
462        return;
463    };
464    let Some((camera, camera_transform)) = cameras.iter().next() else {
465        return;
466    };
467
468    // Figure out where the user clicked on the plane.
469    let Ok(ray) = camera.viewport_to_world(camera_transform, mouse_position) else {
470        return;
471    };
472    let Some(plane_intersection) =
473        ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
474    else {
475        return;
476    };
477    // Move all the main objects.
478    for mut transform in main_objects.iter_mut() {
479        transform.translation = vec3(
480            plane_intersection.x,
481            transform.translation.y,
482            plane_intersection.z,
483        );
484    }
485}
486
487impl FromWorld for ExampleAssets {
488    fn from_world(world: &mut World) -> Self {
489        let fox_animation =
490            world.load_asset(GltfAssetLabel::Animation(1).from_asset("models/animated/Fox.glb"));
491        let (fox_animation_graph, fox_animation_node) =
492            AnimationGraph::from_clip(fox_animation.clone());
493
494        ExampleAssets {
495            main_sphere: world.add_asset(Sphere::default().mesh().uv(32, 18)),
496            fox: world.load_asset(GltfAssetLabel::Scene(0).from_asset("models/animated/Fox.glb")),
497            main_sphere_material: world.add_asset(Color::from(SILVER)),
498            main_scene: world.load_asset(
499                GltfAssetLabel::Scene(0)
500                    .from_asset("models/IrradianceVolumeExample/IrradianceVolumeExample.glb"),
501            ),
502            irradiance_volume: world.load_asset("irradiance_volumes/Example.vxgi.ktx2"),
503            fox_animation_graph: world.add_asset(fox_animation_graph),
504            fox_animation_node,
505            voxel_cube: world.add_asset(Cuboid::default()),
506            // Just use a specular map for the skybox since it's not too blurry.
507            // In reality you wouldn't do this--you'd use a real skybox texture--but
508            // reusing the textures like this saves space in the Bevy repository.
509            skybox: world.load_asset("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
510        }
511    }
512}
513
514// Plays the animation on the fox.
515fn play_animations(
516    mut commands: Commands,
517    assets: Res<ExampleAssets>,
518    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
519) {
520    for (entity, mut player) in players.iter_mut() {
521        commands
522            .entity(entity)
523            .insert(AnimationGraphHandle(assets.fox_animation_graph.clone()));
524        player.play(assets.fox_animation_node).repeat();
525    }
526}
527
528fn create_cubes(
529    image_assets: Res<Assets<Image>>,
530    mut commands: Commands,
531    irradiance_volumes: Query<(&IrradianceVolume, &GlobalTransform)>,
532    voxel_cube_parents: Query<Entity, With<VoxelCubeParent>>,
533    voxel_cubes: Query<Entity, With<VoxelCube>>,
534    example_assets: Res<ExampleAssets>,
535    mut voxel_visualization_material_assets: ResMut<Assets<VoxelVisualizationMaterial>>,
536) {
537    // If voxel cubes have already been spawned, don't do anything.
538    if !voxel_cubes.is_empty() {
539        return;
540    }
541
542    let Some(voxel_cube_parent) = voxel_cube_parents.iter().next() else {
543        return;
544    };
545
546    for (irradiance_volume, global_transform) in irradiance_volumes.iter() {
547        let Some(image) = image_assets.get(&irradiance_volume.voxels) else {
548            continue;
549        };
550
551        let resolution = image.texture_descriptor.size;
552
553        let voxel_cube_material = voxel_visualization_material_assets.add(ExtendedMaterial {
554            base: StandardMaterial::from(Color::from(RED)),
555            extension: VoxelVisualizationExtension {
556                irradiance_volume_info: VoxelVisualizationIrradianceVolumeInfo {
557                    world_from_voxel: VOXEL_FROM_WORLD.inverse(),
558                    voxel_from_world: VOXEL_FROM_WORLD,
559                    resolution: uvec3(
560                        resolution.width,
561                        resolution.height,
562                        resolution.depth_or_array_layers,
563                    ),
564                    intensity: IRRADIANCE_VOLUME_INTENSITY,
565                },
566            },
567        });
568
569        let scale = vec3(
570            1.0 / resolution.width as f32,
571            1.0 / resolution.height as f32,
572            1.0 / resolution.depth_or_array_layers as f32,
573        );
574
575        // Spawn a cube for each voxel.
576        for z in 0..resolution.depth_or_array_layers {
577            for y in 0..resolution.height {
578                for x in 0..resolution.width {
579                    let uvw = (uvec3(x, y, z).as_vec3() + 0.5) * scale - 0.5;
580                    let pos = global_transform.transform_point(uvw);
581                    let voxel_cube = commands
582                        .spawn((
583                            Mesh3d(example_assets.voxel_cube.clone()),
584                            MeshMaterial3d(voxel_cube_material.clone()),
585                            Transform::from_scale(Vec3::splat(VOXEL_CUBE_SCALE))
586                                .with_translation(pos),
587                        ))
588                        .insert(VoxelCube)
589                        .insert(NotShadowCaster)
590                        .id();
591
592                    commands.entity(voxel_cube_parent).add_child(voxel_cube);
593                }
594            }
595        }
596    }
597}
```

examples/2d/mesh2d.rs ([line 22](../../../src/mesh2d/mesh2d.rs.html#22))

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

examples/3d/ssr.rs ([line 284](../../../src/ssr/ssr.rs.html#284))

```rust
278fn spawn_flight_helmet(commands: &mut Commands, asset_server: &AssetServer) {
279    commands.spawn((
280        WorldAssetRoot(
281            asset_server
282                .load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")),
283        ),
284        Transform::from_scale(Vec3::splat(2.5)),
285        FlightHelmetModel,
286        Visibility::Hidden,
287    ));
288}
289
290// Spawns the row of capsules.
291fn spawn_capsules(
292    commands: &mut Commands,
293    meshes: &mut Assets<Mesh>,
294    standard_materials: &mut Assets<StandardMaterial>,
295) {
296    let capsule_mesh = meshes.add(Capsule3d::new(0.4, 0.5));
297    let parent = commands
298        .spawn((
299            Transform::from_xyz(0.0, 0.5, 0.0),
300            Visibility::Hidden,
301            CapsulesParent,
302        ))
303        .id();
304
305    for i in 0..5 {
306        let roughness = i as f32 * 0.25;
307        let child = commands
308            .spawn((
309                Mesh3d(capsule_mesh.clone()),
310                MeshMaterial3d(standard_materials.add(StandardMaterial {
311                    base_color: Color::BLACK,
312                    perceptual_roughness: roughness.max(0.08),
313                    ..default()
314                })),
315                Transform::from_xyz(i as f32 * 1.1 - (1.1 * 2.0), 0.5, 0.0),
316                CapsuleModel,
317            ))
318            .id();
319        commands.entity(parent).add_child(child);
320    }
321}
322
323// Spawns the metallic base.
324fn spawn_metallic_base(
325    commands: &mut Commands,
326    meshes: &mut Assets<Mesh>,
327    standard_materials: &mut Assets<StandardMaterial>,
328) {
329    commands.spawn((
330        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)))),
331        MeshMaterial3d(standard_materials.add(StandardMaterial {
332            base_color: Color::from(bevy::color::palettes::css::DARK_GRAY),
333            metallic: 1.0,
334            perceptual_roughness: 0.3,
335            ..default()
336        })),
337        Transform::from_scale(Vec3::splat(100.0)),
338        MetallicBaseModel,
339        Visibility::Hidden,
340    ));
341}
342
343// Spawns the non-metallic base.
344fn spawn_non_metallic_base(
345    commands: &mut Commands,
346    meshes: &mut Assets<Mesh>,
347    standard_materials: &mut Assets<StandardMaterial>,
348) {
349    commands.spawn((
350        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)))),
351        MeshMaterial3d(standard_materials.add(StandardMaterial {
352            base_color: Color::from(bevy::color::palettes::css::RED),
353            metallic: 0.0,
354            perceptual_roughness: 0.2,
355            ..default()
356        })),
357        Transform::from_scale(Vec3::splat(100.0)),
358        RedPlaneBaseModel,
359        Visibility::Hidden,
360    ));
361}
362
363// Spawns the water plane.
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

examples/movement/physics\_in\_fixed\_timestep.rs ([line 179](../../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#179))

```rust
175fn spawn_player(mut commands: Commands) {
176    commands.spawn((Camera3d::default(), CameraSensitivity::default()));
177    commands.spawn((
178        Name::new("Player"),
179        Transform::from_scale(Vec3::splat(0.3)),
180        AccumulatedInput::default(),
181        Velocity::default(),
182        PhysicalTranslation::default(),
183        PreviousPhysicalTranslation::default(),
184    ));
185}
```

Additional examples can be found in:  

*   [examples/2d/pixel\_grid\_snap.rs](../../../src/pixel_grid_snap/pixel_grid_snap.rs.html#78)
*   [examples/showcase/alien\_cake\_addict.rs](../../../src/alien_cake_addict/alien_cake_addict.rs.html#372)
*   [examples/shader/shader\_material\_2d.rs](../../../src/shader_material_2d/shader_material_2d.rs.html#41)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#230)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#295)
*   [examples/3d/reflection\_probes.rs](../../../src/reflection_probes/reflection_probes.rs.html#166)
*   [examples/3d/pccm.rs](../../../src/pccm/pccm.rs.html#149)
*   [examples/ecs/parallel\_query.rs](../../../src/parallel_query/parallel_query.rs.html#20)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#383)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#68)
*   [examples/2d/sprite\_sheet.rs](../../../src/sprite_sheet/sprite_sheet.rs.html#63)
*   [examples/animation/animation\_graph.rs](../../../src/animation_graph/animation_graph.rs.html#245)
*   [examples/3d/rotate\_environment\_map.rs](../../../src/rotate_environment_map/rotate_environment_map.rs.html#88)
*   [examples/movement/smooth\_follow.rs](../../../src/smooth_follow/smooth_follow.rs.html#113)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#146)
*   [examples/3d/clearcoat.rs](../../../src/clearcoat/clearcoat.rs.html#121)
*   [examples/animation/animated\_mesh\_events.rs](../../../src/animated_mesh_events/animated_mesh_events.rs.html#72)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#381)
*   [examples/3d/atmospheric\_fog.rs](../../../src/atmospheric_fog/atmospheric_fog.rs.html#82)
*   [examples/shader/storage\_buffer.rs](../../../src/storage_buffer/storage_buffer.rs.html#38)
*   [examples/2d/mesh2d\_vertex\_color\_texture.rs](../../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#41)
*   [examples/ecs/hierarchy.rs](../../../src/hierarchy/hierarchy.rs.html#27)
*   [examples/transforms/transform.rs](../../../src/transform/transform.rs.html#149)
*   [examples/stress\_tests/many\_sprites.rs](../../../src/many_sprites/many_sprites.rs.html#78)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../../src/many_sprite_meshes/many_sprite_meshes.rs.html#80)
*   [examples/3d/light\_probe\_blending.rs](../../../src/light_probe_blending/light_probe_blending.rs.html#321)
*   [examples/3d/atmosphere.rs](../../../src/atmosphere/atmosphere.rs.html#247)
*   [examples/3d/spherical\_area\_lights.rs](../../../src/spherical_area_lights/spherical_area_lights.rs.html#58)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../../src/many_animated_sprites/many_animated_sprites.rs.html#76)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#78)
*   [examples/stress\_tests/many\_text2d.rs](../../../src/many_text2d/many_text2d.rs.html#129)
*   [examples/3d/motion\_blur.rs](../../../src/motion_blur/motion_blur.rs.html#82)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#253)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#344)
*   [examples/3d/clustered\_decal\_maps.rs](../../../src/clustered_decal_maps/clustered_decal_maps.rs.html#388)
*   [examples/shader/automatic\_instancing.rs](../../../src/automatic_instancing/automatic_instancing.rs.html#36)
*   [examples/camera/projection\_zoom.rs](../../../src/projection_zoom/projection_zoom.rs.html#85)
*   [examples/3d/fog.rs](../../../src/fog/fog.rs.html#87)
*   [examples/2d/sprite\_animation.rs](../../../src/sprite_animation/sprite_animation.rs.html#123)
*   [examples/3d/volumetric\_fog.rs](../../../src/volumetric_fog/volumetric_fog.rs.html#121)
*   [examples/3d/bloom\_3d.rs](../../../src/bloom_3d/bloom_3d.rs.html#77)
*   [examples/stress\_tests/many\_lights.rs](../../../src/many_lights/many_lights.rs.html#109)
*   [examples/3d/decal.rs](../../../src/decal/decal.rs.html#40)
*   [examples/picking/sprite\_picking.rs](../../../src/sprite_picking/sprite_picking.rs.html#80)
*   [examples/3d/scrolling\_fog.rs](../../../src/scrolling_fog/scrolling_fog.rs.html#112)
*   [examples/ecs/iter\_combinations.rs](../../../src/iter_combinations/iter_combinations.rs.html#86)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#112)
*   [examples/3d/meshlet.rs](../../../src/meshlet/meshlet.rs.html#91)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#93)
*   [examples/2d/texture\_atlas.rs](../../../src/texture_atlas/texture_atlas.rs.html#105)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#184)
*   [examples/3d/deferred\_rendering.rs](../../../src/deferred_rendering/deferred_rendering.rs.html#179)
*   [examples/animation/animated\_transform.rs](../../../src/animated_transform/animated_transform.rs.html#103)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#178)
*   [examples/stress\_tests/many\_cubes.rs](../../../src/many_cubes/many_cubes.rs.html#236)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#110-112)

#### pub fn [map](#method.map)<F>(self, f: F) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html),

Returns a vector containing each element of `self` modified by a mapping function `f`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#124)

#### pub fn [select](#method.select)(mask: [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3"), if\_true: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), if\_false: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Creates a vector from the elements in `if_true` and `if_false`, selecting which to use for each element of `self`.

A true element in the mask uses the corresponding element from `if_true`, and false uses the element from `if_false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#135)

#### pub const fn [from\_array](#method.from_array)(a: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Creates a new vector from an array.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#142)

#### pub const fn [to\_array](#method.to_array)(&self) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts `self` to `[x, y, z]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#153)

#### pub const fn [from\_slice](#method.from_slice)(slice: &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Creates a vector from the first 3 values in `slice`.

##### Panics

Panics if `slice` is less than 3 elements long.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/3d/occlusion\_culling.rs ([line 295](../../../src/occlusion_culling/occlusion_culling.rs.html#295))

```rust
254fn spawn_small_cubes(
255    commands: &mut Commands,
256    meshes: &mut Assets<Mesh>,
257    materials: &mut Assets<StandardMaterial>,
258) {
259    // Add the cube mesh.
260    let small_cube = meshes.add(Cuboid::new(
261        SMALL_CUBE_SIZE,
262        SMALL_CUBE_SIZE,
263        SMALL_CUBE_SIZE,
264    ));
265
266    // Add the cube material.
267    let small_cube_material = materials.add(StandardMaterial {
268        base_color: SILVER.into(),
269        ..default()
270    });
271
272    // Create the entity that the small cubes will be parented to. This is the
273    // entity that we rotate.
274    let sphere_parent = commands
275        .spawn(Transform::from_translation(Vec3::ZERO))
276        .insert(Visibility::default())
277        .insert(SphereParent)
278        .id();
279
280    // Now we have to figure out where to place the cubes. To do that, we create
281    // a sphere mesh, but we don't add it to the scene. Instead, we inspect the
282    // sphere mesh to find the positions of its vertices, and spawn a small cube
283    // at each one. That way, we end up with a bunch of cubes arranged in a
284    // spherical shape.
285
286    // Create the sphere mesh, and extract the positions of its vertices.
287    let sphere = Sphere::new(OUTER_RADIUS)
288        .mesh()
289        .ico(OUTER_SUBDIVISION_COUNT)
290        .unwrap();
291    let sphere_positions = sphere.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
292
293    // At each vertex, create a small cube.
294    for sphere_position in sphere_positions.as_float3().unwrap() {
295        let sphere_position = Vec3::from_slice(sphere_position);
296        let small_cube = commands
297            .spawn(Mesh3d(small_cube.clone()))
298            .insert(MeshMaterial3d(small_cube_material.clone()))
299            .insert(Transform::from_translation(sphere_position))
300            .id();
301        commands.entity(sphere_parent).add_child(small_cube);
302    }
303}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#164)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\])

Writes the elements of `self` to the first 3 elements in `slice`.

##### Panics

Panics if `slice` is less than 3 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#183)

#### pub fn [extend](#method.extend)(self, w: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a 4D vector from `self` and the given `w` value.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/3d/mirror.rs ([line 376](../../../src/mirror/mirror.rs.html#376))

```rust
341fn calculate_mirror_camera_transform_and_projection(
342    main_camera_transform: &Transform,
343    main_camera_projection: &PerspectiveProjection,
344    mirror_transform: &Transform,
345) -> (Transform, PerspectiveProjection) {
346    // Calculate the reflection matrix (a.k.a. Householder matrix) that will
347    // reflect the scene across the mirror plane.
348    //
349    // Note that you must calculate this in *matrix* form and only *afterward*
350    // convert to a `Transform` instead of composing `Transform`s. This is
351    // because the reflection matrix has non-uniform scale, and composing
352    // transforms can't always handle composition of matrices with non-uniform
353    // scales.
354    let mirror_camera_transform = Transform::from_matrix(
355        Mat4::from_mat3a(reflection_matrix(Vec3::NEG_Z)) * main_camera_transform.to_matrix(),
356    );
357
358    // Compute the distance from the camera to the mirror plane. This will be
359    // used to calculate the distance to the near clip plane for the mirror
360    // world.
361    let distance_from_camera_to_mirror = InfinitePlane3d::new(mirror_transform.rotation * Vec3::Y)
362        .signed_distance(
363            Isometry3d::IDENTITY,
364            mirror_transform.translation - main_camera_transform.translation,
365        );
366
367    // Compute the normal of the mirror plane in view space.
368    let view_from_world = main_camera_transform.compute_affine().matrix3.inverse();
369    let mirror_projection_plane_normal =
370        (view_from_world * (mirror_transform.rotation * Vec3::NEG_Y)).normalize();
371
372    // Compute the final projection. It should match the main camera projection,
373    // except that `near` and `near_normal` should be set to the updated near
374    // plane and near normal plane as above.
375    let mirror_camera_projection = PerspectiveProjection {
376        near_clip_plane: mirror_projection_plane_normal.extend(distance_from_camera_to_mirror),
377        ..*main_camera_projection
378    };
379
380    (mirror_camera_transform, mirror_camera_projection)
381}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#192)

#### pub fn [truncate](#method.truncate)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.

Truncation may also be performed by using [`self.xy()`](../../prelude/trait.Vec3Swizzles.html#tymethod.xy "method bevy::prelude::Vec3Swizzles::xy").

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/ecs/observers.rs ([line 204](../../../src/observers/observers.rs.html#204))

```rust
190fn handle_click(
191    mouse_button_input: Res<ButtonInput<MouseButton>>,
192    camera: Single<(&Camera, &GlobalTransform)>,
193    windows: Query<&Window>,
194    mut commands: Commands,
195) {
196    let Ok(windows) = windows.single() else {
197        return;
198    };
199
200    let (camera, camera_transform) = *camera;
201    if let Some(pos) = windows
202        .cursor_position()
203        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
204        .map(|ray| ray.origin.truncate())
205        && mouse_button_input.just_pressed(MouseButton::Left)
206    {
207        commands.trigger(ExplodeMines { pos, radius: 1.0 });
208    }
209}
```

Hide additional examples

examples/2d/mesh2d\_arcs.rs ([line 110](../../../src/mesh2d_arcs/mesh2d_arcs.rs.html#110))

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

examples/showcase/desk\_toy.rs ([line 240](../../../src/desk_toy/desk_toy.rs.html#240))

```rust
220fn update_cursor_hit_test(
221    cursor_world_pos: Res<CursorWorldPos>,
222    primary_window: Single<(&Window, &mut CursorOptions), With<PrimaryWindow>>,
223    bevy_logo_transform: Single<&Transform, With<BevyLogo>>,
224) {
225    let (window, mut cursor_options) = primary_window.into_inner();
226    // If the window has decorations (e.g. a border) then it should be clickable
227    if window.decorations {
228        cursor_options.hit_test = true;
229        return;
230    }
231
232    // If the cursor is not within the window we don't need to update whether the window is clickable or not
233    let Some(cursor_world_pos) = cursor_world_pos.0 else {
234        return;
235    };
236
237    // If the cursor is within the radius of the Bevy logo make the window clickable otherwise the window is not clickable
238    cursor_options.hit_test = bevy_logo_transform
239        .translation
240        .truncate()
241        .distance(cursor_world_pos)
242        < BEVY_LOGO_RADIUS;
243}
244
245/// Start the drag operation and record the offset we started dragging from
246fn start_drag(
247    mut commands: Commands,
248    cursor_world_pos: Res<CursorWorldPos>,
249    bevy_logo_transform: Single<&Transform, With<BevyLogo>>,
250) {
251    // If the cursor is not within the primary window skip this system
252    let Some(cursor_world_pos) = cursor_world_pos.0 else {
253        return;
254    };
255
256    // Get the offset from the cursor to the Bevy logo sprite
257    let drag_offset = bevy_logo_transform.translation.truncate() - cursor_world_pos;
258
259    // If the cursor is within the Bevy logo radius start the drag operation and remember the offset of the cursor from the origin
260    if drag_offset.length() < BEVY_LOGO_RADIUS {
261        commands.insert_resource(DragOperation(drag_offset));
262    }
263}
264
265/// Stop the current drag operation
266fn end_drag(mut commands: Commands) {
267    commands.remove_resource::<DragOperation>();
268}
269
270/// Drag the Bevy logo
271fn drag(
272    drag_offset: Res<DragOperation>,
273    cursor_world_pos: Res<CursorWorldPos>,
274    time: Res<Time>,
275    mut bevy_transform: Single<&mut Transform, With<BevyLogo>>,
276    mut q_pupils: Query<&mut Pupil>,
277) {
278    // If the cursor is not within the primary window skip this system
279    let Some(cursor_world_pos) = cursor_world_pos.0 else {
280        return;
281    };
282
283    // Calculate the new translation of the Bevy logo based on cursor and drag offset
284    let new_translation = cursor_world_pos + drag_offset.0;
285
286    // Calculate how fast we are dragging the Bevy logo (unit/second)
287    let drag_velocity =
288        (new_translation - bevy_transform.translation.truncate()) / time.delta_secs();
289
290    // Update the translation of Bevy logo transform to new translation
291    bevy_transform.translation = new_translation.extend(bevy_transform.translation.z);
292
293    // Add the cursor drag velocity in the opposite direction to each pupil.
294    // Remember pupils are using local coordinates to move. So when the Bevy logo moves right they need to move left to
295    // simulate inertia, otherwise they will move fixed to the parent.
296    for mut pupil in &mut q_pupils {
297        pupil.velocity -= drag_velocity;
298    }
299}
300
301/// Quit when the user right clicks the Bevy logo
302fn quit(
303    cursor_world_pos: Res<CursorWorldPos>,
304    mut app_exit: MessageWriter<AppExit>,
305    bevy_logo_transform: Single<&Transform, With<BevyLogo>>,
306) {
307    // If the cursor is not within the primary window skip this system
308    let Some(cursor_world_pos) = cursor_world_pos.0 else {
309        return;
310    };
311
312    // If the cursor is within the Bevy logo radius send the [`AppExit`] event to quit the app
313    if bevy_logo_transform
314        .translation
315        .truncate()
316        .distance(cursor_world_pos)
317        < BEVY_LOGO_RADIUS
318    {
319        app_exit.write(AppExit::Success);
320    }
321}
322
323/// Enable transparency for the window and make it on top
324fn toggle_transparency(
325    mut commands: Commands,
326    mut window_transparency: ResMut<WindowTransparency>,
327    mut q_instructions_text: Query<&mut Visibility, With<InstructionsText>>,
328    mut primary_window: Single<&mut Window, With<PrimaryWindow>>,
329) {
330    // Toggle the window transparency resource
331    window_transparency.0 = !window_transparency.0;
332
333    // Show or hide the instructions text
334    for mut visibility in &mut q_instructions_text {
335        *visibility = if window_transparency.0 {
336            Visibility::Hidden
337        } else {
338            Visibility::Visible
339        };
340    }
341
342    // Remove the primary window's decorations (e.g. borders), make it always on top of other desktop windows, and set the clear color to transparent
343    // only if window transparency is enabled
344    let clear_color;
345    (
346        primary_window.decorations,
347        primary_window.window_level,
348        clear_color,
349    ) = if window_transparency.0 {
350        (false, WindowLevel::AlwaysOnTop, Color::NONE)
351    } else {
352        (true, WindowLevel::Normal, WINDOW_CLEAR_COLOR)
353    };
354
355    // Set the clear color
356    commands.insert_resource(ClearColor(clear_color));
357}
358
359/// Move the pupils and bounce them around
360fn move_pupils(time: Res<Time>, mut q_pupils: Query<(&mut Pupil, &mut Transform)>) {
361    for (mut pupil, mut transform) in &mut q_pupils {
362        // The wiggle radius is how much the pupil can move within the eye
363        let wiggle_radius = pupil.eye_radius - pupil.pupil_radius;
364        // Store the Z component
365        let z = transform.translation.z;
366        // Truncate the Z component to make the calculations be on [`Vec2`]
367        let mut translation = transform.translation.truncate();
368        // Decay the pupil velocity
369        pupil.velocity *= ops::powf(0.04f32, time.delta_secs());
370        // Move the pupil
371        translation += pupil.velocity * time.delta_secs();
372        // If the pupil hit the outside border of the eye, limit the translation to be within the wiggle radius and invert the velocity.
373        // This is not physically accurate but it's good enough for the googly eyes effect.
374        if translation.length() > wiggle_radius {
375            translation = translation.normalize() * wiggle_radius;
376            // Invert and decrease the velocity of the pupil when it bounces
377            pupil.velocity *= -0.75;
378        }
379        // Update the entity transform with the new translation after reading the Z component
380        transform.translation = translation.extend(z);
381    }
382}
```

examples/ecs/delayed\_commands.rs ([line 49](../../../src/delayed_commands/delayed_commands.rs.html#49))

```rust
34fn click(
35    click: On<Pointer<Click>>,
36    mut commands: Commands,
37    squares: Query<(Entity, &Transform), With<BlinkySquare>>,
38    cameras: Query<(&Camera, &GlobalTransform)>,
39) {
40    let (camera, camera_transform) = cameras.single().unwrap();
41    let mut delayed = commands.delayed();
42    for (entity, transform) in squares.iter() {
43        // convert the pointer position to world position
44        let mouse_world_pos = camera
45            .viewport_to_world_2d(camera_transform, click.pointer_location.position)
46            .unwrap();
47
48        // delay the blinkiness by distance to cursor
49        let dist = mouse_world_pos.distance(transform.translation.truncate());
50        let delay = dist / 1000.0;
51        delayed
52            .secs(delay)
53            .entity(entity)
54            .insert(Sprite::from_color(Color::WHITE, SQUARE_SIZE));
55        delayed
56            .secs(delay + 0.1)
57            .entity(entity)
58            .insert(Sprite::from_color(Color::BLACK, SQUARE_SIZE));
59    }
60}
```

examples/showcase/breakout.rs ([line 344](../../../src/breakout/breakout.rs.html#344))

```rust
334fn check_for_collisions(
335    mut commands: Commands,
336    mut score: ResMut<Score>,
337    ball_query: Single<(&mut Velocity, &Transform), With<Ball>>,
338    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
339) {
340    let (mut ball_velocity, ball_transform) = ball_query.into_inner();
341
342    for (collider_entity, collider_transform, maybe_brick) in &collider_query {
343        let collision = ball_collision(
344            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.),
345            Aabb2d::new(
346                collider_transform.translation.truncate(),
347                collider_transform.scale.truncate() / 2.,
348            ),
349        );
350
351        if let Some(collision) = collision {
352            // Trigger observers of the "BallCollided" event
353            commands.trigger(BallCollided);
354
355            // Bricks should be despawned and increment the scoreboard on collision
356            if maybe_brick.is_some() {
357                commands.entity(collider_entity).despawn();
358                **score += 1;
359            }
360
361            // Reflect the ball's velocity when it collides
362            let mut reflect_x = false;
363            let mut reflect_y = false;
364
365            // Reflect only if the velocity is in the opposite direction of the collision
366            // This prevents the ball from getting stuck inside the bar
367            match collision {
368                Collision::Left => reflect_x = ball_velocity.x > 0.0,
369                Collision::Right => reflect_x = ball_velocity.x < 0.0,
370                Collision::Top => reflect_y = ball_velocity.y < 0.0,
371                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
372            }
373
374            // Reflect velocity on the x-axis if we hit something on the x-axis
375            if reflect_x {
376                ball_velocity.x = -ball_velocity.x;
377            }
378
379            // Reflect velocity on the y-axis if we hit something on the y-axis
380            if reflect_y {
381                ball_velocity.y = -ball_velocity.y;
382            }
383        }
384    }
385}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#204)

#### pub fn [from\_homogeneous](#method.from_homogeneous)(v: [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Projects a homogeneous coordinate to 3D space by performing perspective divide.

##### Panics

Will panic if `v.w` is `0` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#212)

#### pub fn [to\_homogeneous](#method.to_homogeneous)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a homogeneous coordinate from `self`, equivalent to `self.extend(1.0)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#219)

#### pub fn [to\_vec3a](#method.to_vec3a)(self) -> [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#226)

#### pub fn [with\_x](#method.with_x)(self, x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Creates a 3D vector from `self` with the given value of `x`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#234)

#### pub fn [with\_y](#method.with_y)(self, y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Creates a 3D vector from `self` with the given value of `y`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#242)

#### pub fn [with\_z](#method.with_z)(self, z: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Creates a 3D vector from `self` with the given value of `z`.

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/3d/clustered\_decal\_maps.rs ([line 388](../../../src/clustered_decal_maps/clustered_decal_maps.rs.html#388))

```rust
345fn animate_decals(
346    mut commands: Commands,
347    mut decals_query: Query<(Entity, &mut ExampleDecal, &mut Transform)>,
348    time: Res<Time>,
349) {
350    for (decal_entity, mut example_decal, mut decal_transform) in decals_query.iter_mut() {
351        // Update the animation timers, and advance the animation state if the
352        // timer has expired.
353        match example_decal.state {
354            ExampleDecalState::AnimatingIn(ref mut timer) => {
355                timer.tick(time.delta());
356                if timer.just_finished() {
357                    example_decal.state =
358                        ExampleDecalState::Idling(Timer::new(DECAL_IDLE_DURATION, TimerMode::Once));
359                }
360            }
361            ExampleDecalState::Idling(ref mut timer) => {
362                timer.tick(time.delta());
363                if timer.just_finished() {
364                    example_decal.state = ExampleDecalState::AnimatingOut(Timer::new(
365                        DECAL_ANIMATE_OUT_DURATION,
366                        TimerMode::Once,
367                    ));
368                }
369            }
370            ExampleDecalState::AnimatingOut(ref mut timer) => {
371                timer.tick(time.delta());
372                if timer.just_finished() {
373                    commands.entity(decal_entity).despawn();
374                    continue;
375                }
376            }
377        }
378
379        // Actually animate the decal by adjusting its transform.
380        // All we have to do here is to compute the decal's scale as a fraction
381        // of its full size.
382        let new_decal_scale_factor = match example_decal.state {
383            ExampleDecalState::AnimatingIn(ref timer) => timer.fraction(),
384            ExampleDecalState::Idling(_) => 1.0,
385            ExampleDecalState::AnimatingOut(ref timer) => timer.fraction_remaining(),
386        };
387        decal_transform.scale =
388            Vec3::splat(example_decal.size * new_decal_scale_factor).with_z(1.0);
389    }
390}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#250)

#### pub fn [dot](#method.dot)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#257)

#### pub fn [dot\_into\_vec](#method.dot_into_vec)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector where every component is the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#264)

#### pub fn [cross](#method.cross)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Computes the cross product of `self` and `rhs`.

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/ecs/fallible\_params.rs ([line 150](../../../src/fallible_params/fallible_params.rs.html#150))

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#280)

#### pub fn [min](#method.min)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the minimum values for each element of `self` and `rhs`.

In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/2d/rotation.rs ([line 150](../../../src/rotation/rotation.rs.html#150))

```rust
112fn player_movement_system(
113    time: Res<Time>,
114    keyboard_input: Res<ButtonInput<KeyCode>>,
115    query: Single<(&Player, &mut Transform)>,
116) {
117    let (ship, mut transform) = query.into_inner();
118
119    let mut rotation_factor = 0.0;
120    let mut movement_factor = 0.0;
121
122    if keyboard_input.pressed(KeyCode::ArrowLeft) {
123        rotation_factor += 1.0;
124    }
125
126    if keyboard_input.pressed(KeyCode::ArrowRight) {
127        rotation_factor -= 1.0;
128    }
129
130    if keyboard_input.pressed(KeyCode::ArrowUp) {
131        movement_factor += 1.0;
132    }
133
134    // Update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
135    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_secs());
136
137    // Get the ship's forward vector by applying the current rotation to the ships initial facing
138    // vector
139    let movement_direction = transform.rotation * Vec3::Y;
140    // Get the distance the ship will move based on direction, the ship's movement speed and delta
141    // time
142    let movement_distance = movement_factor * ship.movement_speed * time.delta_secs();
143    // Create the change in translation using the new movement direction and distance
144    let translation_delta = movement_direction * movement_distance;
145    // Update the ship translation with our new translation delta
146    transform.translation += translation_delta;
147
148    // Bound the ship within the invisible level bounds
149    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
150    transform.translation = transform.translation.min(extents).max(-extents);
151}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#296)

#### pub fn [max](#method.max)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the maximum values for each element of `self` and `rhs`.

In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

##### [Examples found in repository](#scraped-examples-8)[?](../../../scrape-examples-help.html)

examples/2d/rotation.rs ([line 150](../../../src/rotation/rotation.rs.html#150))

```rust
112fn player_movement_system(
113    time: Res<Time>,
114    keyboard_input: Res<ButtonInput<KeyCode>>,
115    query: Single<(&Player, &mut Transform)>,
116) {
117    let (ship, mut transform) = query.into_inner();
118
119    let mut rotation_factor = 0.0;
120    let mut movement_factor = 0.0;
121
122    if keyboard_input.pressed(KeyCode::ArrowLeft) {
123        rotation_factor += 1.0;
124    }
125
126    if keyboard_input.pressed(KeyCode::ArrowRight) {
127        rotation_factor -= 1.0;
128    }
129
130    if keyboard_input.pressed(KeyCode::ArrowUp) {
131        movement_factor += 1.0;
132    }
133
134    // Update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
135    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_secs());
136
137    // Get the ship's forward vector by applying the current rotation to the ships initial facing
138    // vector
139    let movement_direction = transform.rotation * Vec3::Y;
140    // Get the distance the ship will move based on direction, the ship's movement speed and delta
141    // time
142    let movement_distance = movement_factor * ship.movement_speed * time.delta_secs();
143    // Create the change in translation using the new movement direction and distance
144    let translation_delta = movement_direction * movement_distance;
145    // Update the ship translation with our new translation delta
146    transform.translation += translation_delta;
147
148    // Bound the ship within the invisible level bounds
149    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
150    transform.translation = transform.translation.min(extents).max(-extents);
151}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#316)

#### pub fn [clamp](#method.clamp)(self, min: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), max: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Component-wise clamping of values, similar to [`f32::clamp`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.clamp "method f32::clamp").

Each element in `min` must be less-or-equal to the corresponding element in `max`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

##### Panics

Will panic if `min` is greater than `max` when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-9)[?](../../../scrape-examples-help.html)

examples/3d/light\_textures.rs ([line 533](../../../src/light_textures/light_textures.rs.html#533))

```rust
518fn process_scale_input(
519    mut scale_selections: Query<(&mut Transform, &Selection)>,
520    mut spotlight_selections: Query<(&mut SpotLight, &Selection)>,
521    mouse_buttons: Res<ButtonInput<MouseButton>>,
522    mouse_motion: Res<AccumulatedMouseMotion>,
523    app_status: Res<AppStatus>,
524) {
525    // Only process drags when the scaling operation is selected.
526    if !mouse_buttons.pressed(MouseButton::Left) || app_status.drag_mode != DragMode::Scale {
527        return;
528    }
529
530    for (mut transform, selection) in &mut scale_selections {
531        if app_status.selection == *selection {
532            transform.scale = (transform.scale * (1.0 + mouse_motion.delta.x * SCALE_SPEED))
533                .clamp(Vec3::splat(0.01), Vec3::splat(5.0));
534        }
535    }
536
537    for (mut spotlight, selection) in &mut spotlight_selections {
538        if app_status.selection == *selection {
539            spotlight.outer_angle = (spotlight.outer_angle
540                * (1.0 + mouse_motion.delta.x * SCALE_SPEED))
541                .clamp(0.01, FRAC_PI_4);
542            spotlight.inner_angle = spotlight.outer_angle;
543        }
544    }
545}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#329)

#### pub fn [min\_element](#method.min_element)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the horizontal minimum of `self`.

In other words this computes `min(x, y, ..)`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

##### [Examples found in repository](#scraped-examples-10)[?](../../../scrape-examples-help.html)

examples/transforms/scale.rs ([line 81](../../../src/scale/scale.rs.html#81))

```rust
65fn change_scale_direction(mut cubes: Query<(&mut Transform, &mut Scaling)>) {
66    for (mut transform, mut cube) in &mut cubes {
67        // If an entity scaled beyond the maximum of its size in any dimension
68        // the scaling vector is flipped so the scaling is gradually reverted.
69        // Additionally, to ensure the condition does not trigger again we floor the elements to
70        // their next full value, which should be max_element_size at max.
71        if transform.scale.max_element() > cube.max_element_size {
72            cube.scale_direction *= -1.0;
73            transform.scale = transform.scale.floor();
74        }
75        // If an entity scaled beyond the minimum of its size in any dimension
76        // the scaling vector is also flipped.
77        // Additionally the Values are ceiled to be min_element_size at least
78        // and the scale direction is flipped.
79        // This way the entity will change the dimension in which it is scaled any time it
80        // reaches its min_element_size.
81        if transform.scale.min_element() < cube.min_element_size {
82            cube.scale_direction *= -1.0;
83            transform.scale = transform.scale.ceil();
84            cube.scale_direction = cube.scale_direction.zxy();
85        }
86    }
87}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#342)

#### pub fn [max\_element](#method.max_element)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the horizontal maximum of `self`.

In other words this computes `max(x, y, ..)`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

##### [Examples found in repository](#scraped-examples-11)[?](../../../scrape-examples-help.html)

examples/transforms/scale.rs ([line 71](../../../src/scale/scale.rs.html#71))

```rust
65fn change_scale_direction(mut cubes: Query<(&mut Transform, &mut Scaling)>) {
66    for (mut transform, mut cube) in &mut cubes {
67        // If an entity scaled beyond the maximum of its size in any dimension
68        // the scaling vector is flipped so the scaling is gradually reverted.
69        // Additionally, to ensure the condition does not trigger again we floor the elements to
70        // their next full value, which should be max_element_size at max.
71        if transform.scale.max_element() > cube.max_element_size {
72            cube.scale_direction *= -1.0;
73            transform.scale = transform.scale.floor();
74        }
75        // If an entity scaled beyond the minimum of its size in any dimension
76        // the scaling vector is also flipped.
77        // Additionally the Values are ceiled to be min_element_size at least
78        // and the scale direction is flipped.
79        // This way the entity will change the dimension in which it is scaled any time it
80        // reaches its min_element_size.
81        if transform.scale.min_element() < cube.min_element_size {
82            cube.scale_direction *= -1.0;
83            transform.scale = transform.scale.ceil();
84            cube.scale_direction = cube.scale_direction.zxy();
85        }
86    }
87}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#351)

#### pub fn [min\_position](#method.min_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first minimum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#368)

#### pub fn [max\_position](#method.max_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first maximum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#386)

#### pub fn [element\_sum](#method.element_sum)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the sum of all elements of `self`.

In other words, this computes `self.x + self.y + ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#395)

#### pub fn [element\_product](#method.element_product)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the product of all elements of `self`.

In other words, this computes `self.x * self.y * ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#406)

#### pub fn [cmpeq](#method.cmpeq)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `==` comparison for each element of `self` and `rhs`.

In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#417)

#### pub fn [cmpne](#method.cmpne)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `!=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#428)

#### pub fn [cmpge](#method.cmpge)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `>=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#439)

#### pub fn [cmpgt](#method.cmpgt)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `>` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#450)

#### pub fn [cmple](#method.cmple)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `<=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#461)

#### pub fn [cmplt](#method.cmplt)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `<` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#468)

#### pub fn [abs](#method.abs)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the absolute value of each element of `self`.

##### [Examples found in repository](#scraped-examples-12)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_cubes.rs ([line 609](../../../src/many_cubes/many_cubes.rs.html#609))

```rust
608fn fast_hue_to_rgb(hue: f32) -> Vec3 {
609    (hue * 6.0 - vec3(3.0, 2.0, 4.0)).abs() * vec3(1.0, -1.0, -1.0) + vec3(-1.0, 2.0, 2.0)
610}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#483)

#### pub fn [signum](#method.signum)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector with elements representing the sign of `self`.

*   `1.0` if the number is positive, `+0.0` or `INFINITY`
*   `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
*   `NAN` if the number is `NAN`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#494)

#### pub fn [copysign](#method.copysign)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector with signs of `rhs` and the magnitudes of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#511)

#### pub fn [is\_negative\_bitmask](#method.is_negative_bitmask)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.

A negative element results in a `1` bit and a positive element in a `0` bit. Element `x` goes into the first lowest bit, element `y` into the second, etc.

An element is negative if it has a negative sign, including -0.0, NaNs with negative sign bit and negative infinity.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#521)

#### pub fn [is\_finite](#method.is_finite)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if, and only if, all elements are finite. If any element is either `NaN`, positive or negative infinity, this will return `false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#530)

#### pub fn [is\_finite\_mask](#method.is_finite_mask)(self) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Performs `is_finite` on each element of self, returning a vector mask of the results.

In other words, this computes `[x.is_finite(), y.is_finite(), ...]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#537)

#### pub fn [is\_nan](#method.is_nan)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if any elements are `NaN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#546)

#### pub fn [is\_nan\_mask](#method.is_nan_mask)(self) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Performs `is_nan` on each element of self, returning a vector mask of the results.

In other words, this computes `[x.is_nan(), y.is_nan(), ...]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#554)

#### pub fn [length](#method.length)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the length of `self`.

##### [Examples found in repository](#scraped-examples-13)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 420](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#420))

```rust
419    fn from_cartesian(p: Vec3) -> SphericalCoordinates {
420        let radius = p.length();
421        SphericalCoordinates {
422            radius,
423            inclination: acos(p.y / radius),
424            azimuth: atan2(p.z, p.x),
425        }
426    }
```

Hide additional examples

examples/3d/clustered\_decals.rs ([line 342](../../../src/clustered_decals/clustered_decals.rs.html#342))

```rust
338fn calculate_initial_decal_transform(start: Vec3, looking_at: Vec3, size: Vec2) -> Transform {
339    let direction = looking_at - start;
340    let center = start + direction * 0.5;
341    Transform::from_translation(center)
342        .with_scale((size * 0.5).extend(direction.length()))
343        .looking_to(direction, Vec3::Y)
344}
345
346/// Rotates the cube a bit every frame.
347fn rotate_cube(mut meshes: Query<&mut Transform, With<Mesh3d>>) {
348    for mut transform in &mut meshes {
349        transform.rotate_y(CUBE_ROTATION_SPEED);
350    }
351}
352
353/// Updates the state of the radio buttons when the user clicks on one.
354fn update_radio_buttons(
355    mut widgets: Query<(
356        Entity,
357        Option<&mut BackgroundColor>,
358        Has<Text>,
359        &WidgetClickSender<Selection>,
360    )>,
361    app_status: Res<AppStatus>,
362    mut writer: TextUiWriter,
363) {
364    for (entity, maybe_bg_color, has_text, sender) in &mut widgets {
365        let selected = app_status.selection == **sender;
366        if let Some(mut bg_color) = maybe_bg_color {
367            widgets::update_ui_radio_button(&mut bg_color, selected);
368        }
369        if has_text {
370            widgets::update_ui_radio_button_text(entity, &mut writer, selected);
371        }
372    }
373}
374
375/// Changes the selection when the user clicks a radio button.
376fn handle_selection_change(
377    mut events: MessageReader<WidgetClickEvent<Selection>>,
378    mut app_status: ResMut<AppStatus>,
379) {
380    for event in events.read() {
381        app_status.selection = **event;
382    }
383}
384
385/// Process a drag event that moves the selected object.
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
```

examples/transforms/translation.rs ([line 65](../../../src/translation/translation.rs.html#65))

```rust
62fn move_cube(mut cubes: Query<(&mut Transform, &mut Movable)>, timer: Res<Time>) {
63    for (mut transform, mut cube) in &mut cubes {
64        // Check if the entity moved too far from its spawn, if so invert the moving direction.
65        if (cube.spawn - transform.translation).length() > cube.max_distance {
66            cube.speed *= -1.0;
67        }
68        let direction = transform.local_x();
69        transform.translation += direction * cube.speed * timer.delta_secs();
70    }
71}
```

examples/camera/free\_camera\_controller.rs ([line 197](../../../src/free_camera_controller/free_camera_controller.rs.html#197))

```rust
181fn update_text(
182    mut text_query: Query<&mut Text, With<InfoText>>,
183    camera_query: Query<(&FreeCamera, &FreeCameraState)>,
184) {
185    let mut text = text_query.single_mut().unwrap();
186
187    let (free_camera, free_camera_state) = camera_query.single().unwrap();
188
189    text.0 = format!(
190        "Enabled: {},\nSensitivity: {:.03}\nFriction: {:.01}\nScroll factor: {:.02}\nWalk Speed: {:.02}\nRun Speed: {:.02}\nSpeed: {:.02}",
191        free_camera_state.enabled,
192        free_camera.sensitivity,
193        free_camera.friction,
194        free_camera.scroll_factor,
195        free_camera.walk_speed * free_camera_state.speed_multiplier,
196        free_camera.run_speed * free_camera_state.speed_multiplier,
197        free_camera_state.velocity.length(),
198    );
199}
```

examples/3d/solari.rs ([line 516](../../../src/solari/solari.rs.html#516))

```rust
500fn patrol_path(mut query: Query<(&mut PatrolPath, &mut Transform)>, time: Res<Time<Virtual>>) {
501    for (mut path, mut transform) in query.iter_mut() {
502        let (mut target_position, mut target_rotation) = path.path[path.i];
503        let mut distance_to_target = transform.translation.distance(target_position);
504        if distance_to_target < 0.01 {
505            transform.translation = target_position;
506            transform.rotation = target_rotation;
507
508            path.i = (path.i + 1) % path.path.len();
509            (target_position, target_rotation) = path.path[path.i];
510            distance_to_target = transform.translation.distance(target_position);
511        }
512
513        let direction = (target_position - transform.translation).normalize();
514        let movement = direction * time.delta_secs();
515
516        if movement.length() > distance_to_target {
517            transform.translation = target_position;
518            transform.rotation = target_rotation;
519        } else {
520            transform.translation += movement;
521        }
522    }
523}
```

examples/3d/motion\_blur.rs ([line 324](../../../src/motion_blur/motion_blur.rs.html#324))

```rust
300fn move_cars(
301    time: Res<Time>,
302    mut movables: Query<(&mut Transform, &Moves, &Children)>,
303    mut spins: Query<&mut Transform, (Without<Moves>, With<Rotates>)>,
304) {
305    for (mut transform, moves, children) in &mut movables {
306        let time = time.elapsed_secs() * 0.25;
307        let t = time + 0.5 * moves.0;
308        let dx = ops::cos(t);
309        let dz = -ops::sin(3.0 * t);
310        let speed_variation = (dx * dx + dz * dz).sqrt() * 0.15;
311        let t = t + speed_variation;
312        let prev = transform.translation;
313        transform.translation.x = race_track_pos(0.0, t).x;
314        transform.translation.z = race_track_pos(0.0, t).y;
315        transform.translation.y = -0.59;
316        let delta = transform.translation - prev;
317        transform.look_to(delta, Vec3::Y);
318        for child in children.iter() {
319            let Ok(mut wheel) = spins.get_mut(child) else {
320                continue;
321            };
322            let radius = wheel.scale.x;
323            let circumference = 2.0 * std::f32::consts::PI * radius;
324            let angle = delta.length() / circumference * std::f32::consts::PI * 2.0;
325            wheel.rotate_local_y(angle);
326        }
327    }
328}
```

Additional examples can be found in:  

*   [examples/ecs/fallible\_params.rs](../../../src/fallible_params/fallible_params.rs.html#147)
*   [examples/transforms/transform.rs](../../../src/transform/transform.rs.html#134)
*   [examples/3d/ssr.rs](../../../src/ssr/ssr.rs.html#664)
*   [examples/3d/visibility\_range.rs](../../../src/visibility_range/visibility_range.rs.html#268)
*   [examples/showcase/alien\_cake\_addict.rs](../../../src/alien_cake_addict/alien_cake_addict.rs.html#298)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#487)
*   [examples/3d/light\_probe\_blending.rs](../../../src/light_probe_blending/light_probe_blending.rs.html#521)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#529)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#564)

#### pub fn [length\_squared](#method.length_squared)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the squared length of `self`.

This is faster than `length()` as it avoids a square root operation.

##### [Examples found in repository](#scraped-examples-14)[?](../../../scrape-examples-help.html)

examples/ecs/iter\_combinations.rs ([line 128](../../../src/iter_combinations/iter_combinations.rs.html#128))

```rust
122fn interact_bodies(mut query: Query<(&Mass, &GlobalTransform, &mut Acceleration)>) {
123    let mut iter = query.iter_combinations_mut();
124    while let Some([(Mass(m1), transform1, mut acc1), (Mass(m2), transform2, mut acc2)]) =
125        iter.fetch_next()
126    {
127        let delta = transform2.translation() - transform1.translation();
128        let distance_sq: f32 = delta.length_squared();
129
130        let f = GRAVITY_CONSTANT / distance_sq;
131        let force_unit_mass = delta * f;
132        acc1.0 += force_unit_mass * *m2;
133        acc2.0 -= force_unit_mass * *m1;
134    }
135}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#573)

#### pub fn [length\_recip](#method.length_recip)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes `1.0 / length()`.

For valid results, `self` must _not_ be of length zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#580)

#### pub fn [distance](#method.distance)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the Euclidean distance between two points in space.

##### [Examples found in repository](#scraped-examples-15)[?](../../../scrape-examples-help.html)

examples/3d/solari.rs ([line 503](../../../src/solari/solari.rs.html#503))

```rust
500fn patrol_path(mut query: Query<(&mut PatrolPath, &mut Transform)>, time: Res<Time<Virtual>>) {
501    for (mut path, mut transform) in query.iter_mut() {
502        let (mut target_position, mut target_rotation) = path.path[path.i];
503        let mut distance_to_target = transform.translation.distance(target_position);
504        if distance_to_target < 0.01 {
505            transform.translation = target_position;
506            transform.rotation = target_rotation;
507
508            path.i = (path.i + 1) % path.path.len();
509            (target_position, target_rotation) = path.path[path.i];
510            distance_to_target = transform.translation.distance(target_position);
511        }
512
513        let direction = (target_position - transform.translation).normalize();
514        let movement = direction * time.delta_secs();
515
516        if movement.length() > distance_to_target {
517            transform.translation = target_position;
518            transform.rotation = target_rotation;
519        } else {
520            transform.translation += movement;
521        }
522    }
523}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#587)

#### pub fn [distance\_squared](#method.distance_squared)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Compute the squared euclidean distance between two points in space.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#594)

#### pub fn [div\_euclid](#method.div_euclid)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the element-wise quotient of \[Euclidean division\] of `self` by `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#607)

#### pub fn [rem\_euclid](#method.rem_euclid)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the element-wise remainder of [Euclidean division](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.rem_euclid "method f32::rem_euclid") of `self` by `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#626)

#### pub fn [normalize](#method.normalize)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns `self` normalized to length 1.0.

For valid results, `self` must be finite and _not_ of length zero, nor very close to zero.

See also [`Self::try_normalize()`](../../prelude/struct.Vec3.html#method.try_normalize "method bevy::prelude::Vec3::try_normalize") and [`Self::normalize_or_zero()`](../../prelude/struct.Vec3.html#method.normalize_or_zero "method bevy::prelude::Vec3::normalize_or_zero").

##### Panics

Will panic if the resulting normalized vector is not finite when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-16)[?](../../../scrape-examples-help.html)

examples/picking/mesh\_picking.rs ([line 181](../../../src/mesh_picking/mesh_picking.rs.html#181))

```rust
174fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
175    for (point, normal) in pointers
176        .iter()
177        .filter_map(|interaction| interaction.get_nearest_hit())
178        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
179    {
180        gizmos.sphere(point, 0.05, RED_500);
181        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
182    }
183}
```

Hide additional examples

examples/ui/widgets/viewport\_node.rs ([line 125](../../../src/viewport_node/viewport_node.rs.html#125))

```rust
109fn draw_mesh_intersections(
110    pointers: Query<&PointerInteraction>,
111    untargetable: Query<Entity, Without<Shape>>,
112    mut gizmos: Gizmos,
113) {
114    for (point, normal) in pointers
115        .iter()
116        .flat_map(|interaction| interaction.iter())
117        .filter_map(|(entity, hit)| {
118            if !untargetable.contains(*entity) {
119                hit.position.zip(hit.normal)
120            } else {
121                None
122            }
123        })
124    {
125        gizmos.arrow(point, point + normal.normalize() * 0.5, Color::WHITE);
126    }
127}
```

examples/state/custom\_transitions.rs ([line 191](../../../src/custom_transitions/custom_transitions.rs.html#191))

```rust
170fn movement(
171    time: Res<Time>,
172    input: Res<ButtonInput<KeyCode>>,
173    mut query: Query<&mut Transform, With<Sprite>>,
174) {
175    for mut transform in &mut query {
176        let mut direction = Vec3::ZERO;
177        if input.pressed(KeyCode::ArrowLeft) {
178            direction.x -= 1.0;
179        }
180        if input.pressed(KeyCode::ArrowRight) {
181            direction.x += 1.0;
182        }
183        if input.pressed(KeyCode::ArrowUp) {
184            direction.y += 1.0;
185        }
186        if input.pressed(KeyCode::ArrowDown) {
187            direction.y -= 1.0;
188        }
189
190        if direction != Vec3::ZERO {
191            transform.translation += direction.normalize() * SPEED * time.delta_secs();
192        }
193    }
194}
```

examples/state/states.rs ([line 147](../../../src/states/states.rs.html#147))

```rust
126fn movement(
127    time: Res<Time>,
128    input: Res<ButtonInput<KeyCode>>,
129    mut query: Query<&mut Transform, With<Sprite>>,
130) {
131    for mut transform in &mut query {
132        let mut direction = Vec3::ZERO;
133        if input.pressed(KeyCode::ArrowLeft) {
134            direction.x -= 1.0;
135        }
136        if input.pressed(KeyCode::ArrowRight) {
137            direction.x += 1.0;
138        }
139        if input.pressed(KeyCode::ArrowUp) {
140            direction.y += 1.0;
141        }
142        if input.pressed(KeyCode::ArrowDown) {
143            direction.y -= 1.0;
144        }
145
146        if direction != Vec3::ZERO {
147            transform.translation += direction.normalize() * SPEED * time.delta_secs();
148        }
149    }
150}
```

examples/state/sub\_states.rs ([line 112](../../../src/sub_states/sub_states.rs.html#112))

```rust
91fn movement(
92    time: Res<Time>,
93    input: Res<ButtonInput<KeyCode>>,
94    mut query: Query<&mut Transform, With<Sprite>>,
95) {
96    for mut transform in &mut query {
97        let mut direction = Vec3::ZERO;
98        if input.pressed(KeyCode::ArrowLeft) {
99            direction.x -= 1.0;
100        }
101        if input.pressed(KeyCode::ArrowRight) {
102            direction.x += 1.0;
103        }
104        if input.pressed(KeyCode::ArrowUp) {
105            direction.y += 1.0;
106        }
107        if input.pressed(KeyCode::ArrowDown) {
108            direction.y -= 1.0;
109        }
110
111        if direction != Vec3::ZERO {
112            transform.translation += direction.normalize() * SPEED * time.delta_secs();
113        }
114    }
115}
```

examples/picking/custom\_hit\_data.rs ([line 189](../../../src/custom_hit_data/custom_hit_data.rs.html#189))

```rust
185fn draw_hit_gizmos(hovered_triangles: Res<HoveredTriangles>, mut gizmos: Gizmos) {
186    for triangle in &hovered_triangles.0 {
187        gizmos.arrow(
188            triangle.position,
189            triangle.position + triangle.normal.normalize() * 0.5,
190            WHITE,
191        );
192
193        let vertices = triangle.vertices;
194        let center = (vertices[0] + vertices[1] + vertices[2]) / 3.0;
195        let offset = triangle.normal.normalize_or_zero() * 0.025;
196
197        // The outline is made bigger and offset a bit to prevent being covered
198        // by the mesh
199        let outline = vertices.map(|vertex| center + (vertex - center) * 1.05 + offset);
200
201        gizmos.line(outline[0], outline[1], WHITE);
202        gizmos.line(outline[1], outline[2], WHITE);
203        gizmos.line(outline[2], outline[0], WHITE);
204    }
205}
```

Additional examples can be found in:  

*   [examples/state/computed\_states.rs](../../../src/computed_states/computed_states.rs.html#441)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#513)
*   [examples/3d/mirror.rs](../../../src/mirror/mirror.rs.html#370)
*   [examples/ecs/iter\_combinations.rs](../../../src/iter_combinations/iter_combinations.rs.html#61)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#181)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#641)

#### pub fn [try\_normalize](#method.try_normalize)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>

Returns `self` normalized to length 1.0 if possible, else returns `None`.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be `None`.

See also [`Self::normalize_or_zero()`](../../prelude/struct.Vec3.html#method.normalize_or_zero "method bevy::prelude::Vec3::normalize_or_zero").

##### [Examples found in repository](#scraped-examples-17)[?](../../../scrape-examples-help.html)

examples/math/render\_primitives.rs ([line 662](../../../src/render_primitives/render_primitives.rs.html#662))

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#659)

#### pub fn [normalize\_or](#method.normalize_or)(self, fallback: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns `self` normalized to length 1.0 if possible, else returns a fallback value.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be the fallback value.

See also [`Self::try_normalize()`](../../prelude/struct.Vec3.html#method.try_normalize "method bevy::prelude::Vec3::try_normalize").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#676)

#### pub fn [normalize\_or\_zero](#method.normalize_or_zero)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns `self` normalized to length 1.0 if possible, else returns zero.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be zero.

See also [`Self::try_normalize()`](../../prelude/struct.Vec3.html#method.try_normalize "method bevy::prelude::Vec3::try_normalize").

##### [Examples found in repository](#scraped-examples-18)[?](../../../scrape-examples-help.html)

examples/3d/mirror.rs ([line 195](../../../src/mirror/mirror.rs.html#195))

```rust
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
```

Hide additional examples

examples/picking/custom\_hit\_data.rs ([line 195](../../../src/custom_hit_data/custom_hit_data.rs.html#195))

```rust
185fn draw_hit_gizmos(hovered_triangles: Res<HoveredTriangles>, mut gizmos: Gizmos) {
186    for triangle in &hovered_triangles.0 {
187        gizmos.arrow(
188            triangle.position,
189            triangle.position + triangle.normal.normalize() * 0.5,
190            WHITE,
191        );
192
193        let vertices = triangle.vertices;
194        let center = (vertices[0] + vertices[1] + vertices[2]) / 3.0;
195        let offset = triangle.normal.normalize_or_zero() * 0.025;
196
197        // The outline is made bigger and offset a bit to prevent being covered
198        // by the mesh
199        let outline = vertices.map(|vertex| center + (vertex - center) * 1.05 + offset);
200
201        gizmos.line(outline[0], outline[1], WHITE);
202        gizmos.line(outline[1], outline[2], WHITE);
203        gizmos.line(outline[2], outline[0], WHITE);
204    }
205}
```

examples/3d/ssr.rs ([line 662](../../../src/ssr/ssr.rs.html#662))

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

examples/3d/visibility\_range.rs ([line 267](../../../src/visibility_range/visibility_range.rs.html#267))

```rust
237fn move_camera(
238    keyboard_input: Res<ButtonInput<KeyCode>>,
239    mut mouse_wheel_reader: MessageReader<MouseWheel>,
240    mut cameras: Query<&mut Transform, With<Camera3d>>,
241) {
242    let (mut zoom_delta, mut theta_delta) = (0.0, 0.0);
243
244    // Process zoom in and out via the keyboard.
245    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
246        zoom_delta -= CAMERA_KEYBOARD_ZOOM_SPEED;
247    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
248        zoom_delta += CAMERA_KEYBOARD_ZOOM_SPEED;
249    }
250
251    // Process left and right pan via the keyboard.
252    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
253        theta_delta -= CAMERA_KEYBOARD_PAN_SPEED;
254    } else if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
255        theta_delta += CAMERA_KEYBOARD_PAN_SPEED;
256    }
257
258    // Process zoom in and out via the mouse wheel.
259    for mouse_wheel in mouse_wheel_reader.read() {
260        zoom_delta -= mouse_wheel.y * CAMERA_MOUSE_MOVEMENT_SPEED;
261    }
262
263    // Update the camera transform.
264    for transform in cameras.iter_mut() {
265        let transform = transform.into_inner();
266
267        let direction = transform.translation.normalize_or_zero();
268        let magnitude = transform.translation.length();
269
270        let new_direction = Mat3::from_rotation_y(theta_delta) * direction;
271        let new_magnitude = (magnitude + zoom_delta).max(MIN_ZOOM_DISTANCE);
272
273        transform.translation = new_direction * new_magnitude;
274        transform.look_at(CAMERA_FOCAL_POINT, Vec3::Y);
275    }
276}
```

examples/3d/decal.rs ([line 75](../../../src/decal/decal.rs.html#75))

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#685)

#### pub fn [normalize\_and\_length](#method.normalize_and_length)(self) -> ([Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Returns `self` normalized to length 1.0 and the length of `self`.

If `self` is zero length then `(Self::X, 0.0)` is returned.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#700)

#### pub fn [is\_normalized](#method.is_normalized)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns whether `self` is length `1.0` or not.

Uses a precision threshold of approximately `1e-4`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#713)

#### pub fn [project\_onto](#method.project_onto)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the vector projection of `self` onto `rhs`.

`rhs` must be of non-zero length.

##### Panics

Will panic if `rhs` is zero length when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#732)

#### pub fn [reject\_from](#method.reject_from)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the vector rejection of `self` from `rhs`.

The vector rejection is the vector perpendicular to the projection of `self` onto `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.

`rhs` must be of non-zero length.

##### Panics

Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#745)

#### pub fn [project\_onto\_normalized](#method.project_onto_normalized)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the vector projection of `self` onto `rhs`.

`rhs` must be normalized.

##### Panics

Will panic if `rhs` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#763)

#### pub fn [reject\_from\_normalized](#method.reject_from_normalized)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the vector rejection of `self` from `rhs`.

The vector rejection is the vector perpendicular to the projection of `self` onto `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.

`rhs` must be normalized.

##### Panics

Will panic if `rhs` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#771)

#### pub fn [round](#method.round)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the nearest integer to a number for each element of `self`. Round half-way cases away from 0.0.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#783)

#### pub fn [floor](#method.floor)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the largest integer less than or equal to a number for each element of `self`.

##### [Examples found in repository](#scraped-examples-19)[?](../../../scrape-examples-help.html)

examples/transforms/scale.rs ([line 73](../../../src/scale/scale.rs.html#73))

```rust
65fn change_scale_direction(mut cubes: Query<(&mut Transform, &mut Scaling)>) {
66    for (mut transform, mut cube) in &mut cubes {
67        // If an entity scaled beyond the maximum of its size in any dimension
68        // the scaling vector is flipped so the scaling is gradually reverted.
69        // Additionally, to ensure the condition does not trigger again we floor the elements to
70        // their next full value, which should be max_element_size at max.
71        if transform.scale.max_element() > cube.max_element_size {
72            cube.scale_direction *= -1.0;
73            transform.scale = transform.scale.floor();
74        }
75        // If an entity scaled beyond the minimum of its size in any dimension
76        // the scaling vector is also flipped.
77        // Additionally the Values are ceiled to be min_element_size at least
78        // and the scale direction is flipped.
79        // This way the entity will change the dimension in which it is scaled any time it
80        // reaches its min_element_size.
81        if transform.scale.min_element() < cube.min_element_size {
82            cube.scale_direction *= -1.0;
83            transform.scale = transform.scale.ceil();
84            cube.scale_direction = cube.scale_direction.zxy();
85        }
86    }
87}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#795)

#### pub fn [ceil](#method.ceil)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the smallest integer greater than or equal to a number for each element of `self`.

##### [Examples found in repository](#scraped-examples-20)[?](../../../scrape-examples-help.html)

examples/transforms/scale.rs ([line 83](../../../src/scale/scale.rs.html#83))

```rust
65fn change_scale_direction(mut cubes: Query<(&mut Transform, &mut Scaling)>) {
66    for (mut transform, mut cube) in &mut cubes {
67        // If an entity scaled beyond the maximum of its size in any dimension
68        // the scaling vector is flipped so the scaling is gradually reverted.
69        // Additionally, to ensure the condition does not trigger again we floor the elements to
70        // their next full value, which should be max_element_size at max.
71        if transform.scale.max_element() > cube.max_element_size {
72            cube.scale_direction *= -1.0;
73            transform.scale = transform.scale.floor();
74        }
75        // If an entity scaled beyond the minimum of its size in any dimension
76        // the scaling vector is also flipped.
77        // Additionally the Values are ceiled to be min_element_size at least
78        // and the scale direction is flipped.
79        // This way the entity will change the dimension in which it is scaled any time it
80        // reaches its min_element_size.
81        if transform.scale.min_element() < cube.min_element_size {
82            cube.scale_direction *= -1.0;
83            transform.scale = transform.scale.ceil();
84            cube.scale_direction = cube.scale_direction.zxy();
85        }
86    }
87}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#807)

#### pub fn [trunc](#method.trunc)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the integer part each element of `self`. This means numbers are always truncated towards zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#820)

#### pub fn [step](#method.step)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing `0.0` if `rhs < self` and 1.0 otherwise.

Similar to glsl’s step(edge, x), which translates into edge.step(x)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#827)

#### pub fn [saturate](#method.saturate)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing all elements of `self` clamped to the range of `[0, 1]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#839)

#### pub fn [fract](#method.fract)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the fractional part of the vector as `self - self.trunc()`.

Note that this differs from the GLSL implementation of `fract` which returns `self - self.floor()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#851)

#### pub fn [fract\_gl](#method.fract_gl)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the fractional part of the vector as `self - self.floor()`.

Note that this differs from the Rust implementation of `fract` which returns `self - self.trunc()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#859)

#### pub fn [exp](#method.exp)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing `e^self` (the exponential function) for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#866)

#### pub fn [exp2](#method.exp2)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing `2^self` for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#874)

#### pub fn [ln](#method.ln)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the natural logarithm for each element of `self`. This returns NaN when the element is negative and negative infinity when the element is zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#882)

#### pub fn [log2](#method.log2)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the base 2 logarithm for each element of `self`. This returns NaN when the element is negative and negative infinity when the element is zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#889)

#### pub fn [powf](#method.powf)(self, n: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing each element of `self` raised to the power of `n`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#901)

#### pub fn [sqrt](#method.sqrt)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the square root for each element of `self`. This returns NaN when the element is negative.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#908)

#### pub fn [cos](#method.cos)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the cosine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#915)

#### pub fn [sin](#method.sin)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the sine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#922)

#### pub fn [sin\_cos](#method.sin_cos)(self) -> ([Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Returns a tuple of two vectors containing the sine and cosine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#936)

#### pub fn [recip](#method.recip)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#952)

#### pub fn [lerp](#method.lerp)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), s: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs a linear interpolation between `self` and `rhs` based on the value `s`.

When `s` is `0.0`, the result will be equal to `self`. When `s` is `1.0`, the result will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly extrapolated.

##### [Examples found in repository](#scraped-examples-21)[?](../../../scrape-examples-help.html)

examples/gizmos/axes.rs ([line 211](../../../src/axes/axes.rs.html#211))

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

examples/3d/parallax\_mapping.rs ([line 196](../../../src/parallax_mapping/parallax_mapping.rs.html#196))

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

examples/movement/physics\_in\_fixed\_timestep.rs ([line 406](../../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#406))

```rust
390fn interpolate_rendered_transform(
391    fixed_time: Res<Time<Fixed>>,
392    mut query: Query<(
393        &mut Transform,
394        &PhysicalTranslation,
395        &PreviousPhysicalTranslation,
396    )>,
397) {
398    for (mut transform, current_physical_translation, previous_physical_translation) in
399        query.iter_mut()
400    {
401        let previous = previous_physical_translation.0;
402        let current = current_physical_translation.0;
403        // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
404        let alpha = fixed_time.overstep_fraction();
405
406        let rendered_translation = previous.lerp(current, alpha);
407        transform.translation = rendered_translation;
408    }
409}
```

examples/showcase/alien\_cake\_addict.rs ([line 283](../../../src/alien_cake_addict/alien_cake_addict.rs.html#283))

```rust
268fn focus_camera(
269    time: Res<Time>,
270    mut game: ResMut<Game>,
271    mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
272) {
273    const SPEED: f32 = 2.0;
274    // if there is both a player and a bonus, target the mid-point of them
275    if let (Some(player_entity), Some(bonus_entity)) = (game.player.entity, game.bonus.entity) {
276        let transform_query = transforms.p1();
277        if let (Ok(player_transform), Ok(bonus_transform)) = (
278            transform_query.get(player_entity),
279            transform_query.get(bonus_entity),
280        ) {
281            game.camera_should_focus = player_transform
282                .translation
283                .lerp(bonus_transform.translation, 0.5);
284        }
285        // otherwise, if there is only a player, target the player
286    } else if let Some(player_entity) = game.player.entity {
287        if let Ok(player_transform) = transforms.p1().get(player_entity) {
288            game.camera_should_focus = player_transform.translation;
289        }
290        // otherwise, target the middle
291    } else {
292        game.camera_should_focus = Vec3::from(RESET_FOCUS);
293    }
294    // calculate the camera motion based on the difference between where the camera is looking
295    // and where it should be looking; the greater the distance, the faster the motion;
296    // smooth out the camera movement using the frame time
297    let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
298    if camera_motion.length() > 0.2 {
299        camera_motion *= SPEED * time.delta_secs();
300        // set the new camera's actual focus
301        game.camera_is_focus += camera_motion;
302    }
303    // look at that new camera's actual focus
304    for mut transform in transforms.p0().iter_mut() {
305        *transform = transform.looking_at(game.camera_is_focus, Vec3::Y);
306    }
307}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#962)

#### pub fn [move\_towards](#method.move_towards)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), d: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Moves towards `rhs` based on the value `d`.

When `d` is `0.0`, the result will be equal to `self`. When `d` is equal to `self.distance(rhs)`, the result will be equal to `rhs`. Will not go past `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#977)

#### pub fn [midpoint](#method.midpoint)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Calculates the midpoint between `self` and `rhs`.

The midpoint is the average of, or halfway point between, two vectors. `a.midpoint(b)` should yield the same result as `a.lerp(b, 0.5)` while being slightly cheaper to compute.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#992)

#### pub fn [abs\_diff\_eq](#method.abs_diff_eq)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), max\_abs\_diff: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the absolute difference of all elements between `self` and `rhs` is less than or equal to `max_abs_diff`.

This can be used to compare if two vectors contain similar elements. It works best when comparing with a known value. The `max_abs_diff` that should be used used depends on the values being compared against.

For more see [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1003)

#### pub fn [clamp\_length](#method.clamp_length)(self, min: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), max: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector with a length no less than `min` and no more than `max`.

##### Panics

Will panic if `min` is greater than `max`, or if either `min` or `max` is negative, when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1023)

#### pub fn [clamp\_length\_max](#method.clamp_length_max)(self, max: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector with a length no more than `max`.

##### Panics

Will panic if `max` is negative when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-22)[?](../../../scrape-examples-help.html)

examples/movement/physics\_in\_fixed\_timestep.rs ([line 338](../../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#338))

```rust
298fn accumulate_input(
299    keyboard_input: Res<ButtonInput<KeyCode>>,
300    player: Single<(&mut AccumulatedInput, &mut Velocity)>,
301    camera: Single<&Transform, With<Camera>>,
302) {
303    /// Since Bevy's 3D renderer assumes SI units, this has the unit of meters per second.
304    /// Note that about 1.5 is the average walking speed of a human.
305    const SPEED: f32 = 4.0;
306    let (mut input, mut velocity) = player.into_inner();
307    // Reset the input to zero before reading the new input. As mentioned above, we can only do this
308    // because this is continuously pressed by the user. Do not reset e.g. whether the user wants to boost.
309    input.movement = Vec2::ZERO;
310    if keyboard_input.pressed(KeyCode::KeyW) {
311        input.movement.y += 1.0;
312    }
313    if keyboard_input.pressed(KeyCode::KeyS) {
314        input.movement.y -= 1.0;
315    }
316    if keyboard_input.pressed(KeyCode::KeyA) {
317        input.movement.x -= 1.0;
318    }
319    if keyboard_input.pressed(KeyCode::KeyD) {
320        input.movement.x += 1.0;
321    }
322
323    // Remap the 2D input to Bevy's 3D coordinate system.
324    // Pressing W makes `input.y` go up. Since Bevy assumes that -Z is forward, we make our new Z equal to -input.y
325    let input_3d = Vec3 {
326        x: input.movement.x,
327        y: 0.0,
328        z: -input.movement.y,
329    };
330
331    // Rotate the input so that forward is aligned with the camera's forward direction.
332    let rotated_input = camera.rotation * input_3d;
333
334    // We need to normalize and scale because otherwise
335    // diagonal movement would be faster than horizontal or vertical movement.
336    // We use `clamp_length_max` instead of `.normalize_or_zero()` because gamepad input
337    // may be smaller than 1.0 when the player is pushing the stick just a little bit.
338    velocity.0 = rotated_input.clamp_length_max(1.0) * SPEED;
339}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1040)

#### pub fn [clamp\_length\_min](#method.clamp_length_min)(self, min: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a vector with a length no less than `min`.

##### Panics

Will panic if `min` is negative when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1059)

#### pub fn [mul\_add](#method.mul_add)(self, a: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), b: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding error, yielding a more accurate result than an unfused multiply-add.

Using `mul_add` _may_ be more performant than an unfused multiply-add if the target architecture has a dedicated fma CPU instruction. However, this is not always true, and will be heavily dependant on designing algorithms with specific target hardware in mind.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1077)

#### pub fn [reflect](#method.reflect)(self, normal: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the reflection vector for a given incident vector `self` and surface normal `normal`.

`normal` must be normalized.

##### Panics

Will panic if `normal` is not normalized when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-23)[?](../../../scrape-examples-help.html)

examples/3d/mesh\_ray\_cast.rs ([line 64](../../../src/mesh_ray_cast/mesh_ray_cast.rs.html#64))

```rust
45fn bounce_ray(mut ray: Ray3d, ray_cast: &mut MeshRayCast, gizmos: &mut Gizmos, color: Color) {
46    let mut intersections = Vec::with_capacity(MAX_BOUNCES + 1);
47    intersections.push((ray.origin, Color::srgb(30.0, 0.0, 0.0)));
48
49    for i in 0..MAX_BOUNCES {
50        // Cast the ray and get the first hit
51        let Some((_, hit)) = ray_cast
52            .cast_ray(ray, &MeshRayCastSettings::default())
53            .first()
54        else {
55            break;
56        };
57
58        // Draw the point of intersection and add it to the list
59        let brightness = 1.0 + 10.0 * (1.0 - i as f32 / MAX_BOUNCES as f32);
60        intersections.push((hit.point, Color::BLACK.mix(&color, brightness)));
61        gizmos.sphere(hit.point, 0.005, Color::BLACK.mix(&color, brightness * 2.0));
62
63        // Reflect the ray off of the surface
64        ray.direction = Dir3::new(ray.direction.reflect(hit.normal)).unwrap();
65        ray.origin = hit.point + ray.direction * 1e-6;
66    }
67    gizmos.linestrip_gradient(intersections);
68}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1093)

#### pub fn [refract](#method.refract)(self, normal: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), eta: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the refraction direction for a given incident vector `self`, surface normal `normal` and ratio of indices of refraction, `eta`. When total internal reflection occurs, a zero vector will be returned.

`self` and `normal` must be normalized.

##### Panics

Will panic if `self` or `normal` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1110)

#### pub fn [angle\_between](#method.angle_between)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the angle (in radians) between two vectors in the range `[0, +π]`.

The inputs do not need to be unit vectors however they must be non-zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1120)

#### pub fn [rotate\_x](#method.rotate_x)(self, angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Rotates around the x axis by `angle` (in radians).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1132)

#### pub fn [rotate\_y](#method.rotate_y)(self, angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Rotates around the y axis by `angle` (in radians).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1144)

#### pub fn [rotate\_z](#method.rotate_z)(self, angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Rotates around the z axis by `angle` (in radians).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1162)

#### pub fn [rotate\_axis](#method.rotate_axis)(self, axis: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Rotates around `axis` by `angle` (in radians).

The axis must be a unit vector.

##### Panics

Will panic if `axis` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1173)

#### pub fn [rotate\_towards](#method.rotate_towards)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), max\_angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Rotates towards `rhs` up to `max_angle` (in radians).

When `max_angle` is `0.0`, the result will be equal to `self`. When `max_angle` is equal to `self.angle_between(rhs)`, the result will be parallel to `rhs`. If `max_angle` is negative, rotates towards the exact opposite of `rhs`. Will not go past the target.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1192)

#### pub fn [any\_orthogonal\_vector](#method.any_orthogonal_vector)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns some vector that is orthogonal to the given one.

The input vector must be finite and non-zero.

The output vector is not necessarily unit length. For that use [`Self::any_orthonormal_vector()`](../../prelude/struct.Vec3.html#method.any_orthonormal_vector "method bevy::prelude::Vec3::any_orthonormal_vector") instead.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1210)

#### pub fn [any\_orthonormal\_vector](#method.any_orthonormal_vector)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns any unit vector that is orthogonal to the given one.

The input vector must be unit length.

##### Panics

Will panic if `self` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1227)

#### pub fn [any\_orthonormal\_pair](#method.any_orthonormal_pair)(self) -> ([Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Given a unit vector return two other vectors that together form an orthonormal basis. That is, all three vectors are orthogonal to each other and are normalized.

##### Panics

Will panic if `self` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1246)

#### pub fn [slerp](#method.slerp)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), s: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs a spherical linear interpolation between `self` and `rhs` based on the value `s`.

When `s` is `0.0`, the result will be equal to `self`. When `s` is `1.0`, the result will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly extrapolated.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1285)

#### pub fn [as\_dvec3](#method.as_dvec3)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Casts all elements of `self` to `f64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1292)

#### pub fn [as\_i8vec3](#method.as_i8vec3)(self) -> [I8Vec3](../struct.I8Vec3.html "struct bevy::math::I8Vec3")

Casts all elements of `self` to `i8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1299)

#### pub fn [as\_u8vec3](#method.as_u8vec3)(self) -> [U8Vec3](../struct.U8Vec3.html "struct bevy::math::U8Vec3")

Casts all elements of `self` to `u8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1306)

#### pub fn [as\_i16vec3](#method.as_i16vec3)(self) -> [I16Vec3](../struct.I16Vec3.html "struct bevy::math::I16Vec3")

Casts all elements of `self` to `i16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1313)

#### pub fn [as\_u16vec3](#method.as_u16vec3)(self) -> [U16Vec3](../struct.U16Vec3.html "struct bevy::math::U16Vec3")

Casts all elements of `self` to `u16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1320)

#### pub fn [as\_ivec3](#method.as_ivec3)(self) -> [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

Casts all elements of `self` to `i32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1327)

#### pub fn [as\_uvec3](#method.as_uvec3)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

Casts all elements of `self` to `u32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1334)

#### pub fn [as\_i64vec3](#method.as_i64vec3)(self) -> [I64Vec3](../struct.I64Vec3.html "struct bevy::math::I64Vec3")

Casts all elements of `self` to `i64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1341)

#### pub fn [as\_u64vec3](#method.as_u64vec3)(self) -> [U64Vec3](../struct.U64Vec3.html "struct bevy::math::U64Vec3")

Casts all elements of `self` to `u64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1348)

#### pub fn [as\_isizevec3](#method.as_isizevec3)(self) -> [ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

Casts all elements of `self` to `isize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1355)

#### pub fn [as\_usizevec3](#method.as_usizevec3)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Casts all elements of `self` to `usize`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1647)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1648)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1650)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1659)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1660)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1662)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1667)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1668)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1670)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1763)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1764)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1766)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1771)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1772)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1774)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1711)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1712)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1714)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1719)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1720)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1722)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1675)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1676)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1678)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1751)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1752)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1754)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1779)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1780)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1782)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1699)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1700)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1702)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1727)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1728)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1730)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1683)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1685)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1692)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1694)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1744)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1746)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1735)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1737)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#99)

### impl [Animatable](../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#101)

#### fn [interpolate](../../prelude/trait.Animatable.html#tymethod.interpolate)(a: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), b: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Interpolates between `a` and `b` with an interpolation factor of `time`. [Read more](../../prelude/trait.Animatable.html#tymethod.interpolate)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#106)

#### fn [blend](../../prelude/trait.Animatable.html#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](../../prelude/struct.BlendInput.html "struct bevy::prelude::BlendInput")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>>) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Blends one or more values together. [Read more](../../prelude/trait.Animatable.html#tymethod.blend)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2074)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2076)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [AsMutVectorParts](../../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

#### fn [as\_mut\_parts](../../render/render_resource/encase/vector/trait.AsMutVectorParts.html#tymethod.as_mut_parts)(&mut self) -> &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2067)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2069)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [AsRefVectorParts](../../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

#### fn [as\_ref\_parts](../../render/render_resource/encase/vector/trait.AsRefVectorParts.html#tymethod.as_ref_parts)(&self) -> &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#20)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#20)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#20)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [CreateFrom](../../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [FromVectorParts](../../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](../../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

#### fn [create\_from](../../render/render_resource/encase/internal/trait.CreateFrom.html#tymethod.create_from)<B>(reader: &mut [Reader](../../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where B: [BufferRef](../../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2176)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2177)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1360)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1362)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Deserialize expects a sequence of 3 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2166)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2167)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/mesh_sampling.rs.html#37)

### impl [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [UniformMeshSampler](../sampling/struct.UniformMeshSampler.html "struct bevy::math::sampling::UniformMeshSampler")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/mesh_sampling.rs.html#38)

#### fn [sample](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html#tymethod.sample)<R>(&self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where R: [RngExt](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Generate a random value of `T`, using `rng` as the source of randomness.

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/distr/distribution.rs.html#75-78)

#### fn [sample\_iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html#method.sample_iter)<R>(self, rng: R) -> [Iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html "struct rand::distr::distribution::Iter")<Self, R, T> [ⓘ](#)

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Create an iterator that generates random values of `T`, using `rng` as the source of randomness. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html#method.sample_iter)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/distr/distribution.rs.html#100-103)

#### fn [map](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html#method.map)<F, S>(self, func: F) -> [Map](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/struct.Map.html "struct rand::distr::distribution::Map")<Self, F, T, S>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(T) -> S, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Map sampled values to type `S` [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html#method.map)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1367)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1368)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1370)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1379)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1380)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1382)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1387)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1388)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1390)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1483)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1484)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1486)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1491)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1492)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1494)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1431)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1432)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1434)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1439)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1440)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1442)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1395)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1396)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1398)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1471)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1472)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1474)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1499)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1500)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1502)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1419)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1420)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1422)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1447)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1448)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1450)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1403)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1405)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1412)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1414)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1464)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1466)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1455)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1457)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2214)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2216)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2200)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2202)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: ([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2221)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2223)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2228)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec3A](../../prelude/struct.BVec3A.html "struct bevy::prelude::BVec3A")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2230)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec3A](../../prelude/struct.BVec3A.html "struct bevy::prelude::BVec3A")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#395)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Dir3](../../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#395)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Dir3](../../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2193)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2195)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec3a.rs.html#2173)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec3a.rs.html#2175)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2221)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2223)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#483)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Isometry3d](../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#485)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(translation: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Isometry3d](../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec3a.rs.html#2180)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec3a.rs.html#2182)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2186)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2188)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(a: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### type [This](../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The type to convert into. [Read more](../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [from\_arg](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3") as [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#640)

### impl [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Polyline3d](../../prelude/struct.Polyline3d.html "struct bevy::prelude::Polyline3d")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#641)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iter: I) -> [Polyline3d](../../prelude/struct.Polyline3d.html "struct bevy::prelude::Polyline3d")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [from\_reflect](../../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [FromVectorParts](../../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

#### fn [from\_parts](../../render/render_resource/encase/vector/trait.FromVectorParts.html#tymethod.from_parts)(parts: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [GetOwnership](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [ownership](../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [get\_type\_registration](../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [register\_type\_dependencies](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2141)

### impl [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2142)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The returned type after indexing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2144)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2154)

### impl [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2156)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &mut <[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [IntoReturn](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [into\_return](../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): 'into\_return,

Converts [`Self`](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1507)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1508)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1510)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#985)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#986)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#988)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#993)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#994)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#996)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1519)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1520)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1522)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1527)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1528)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1530)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1623)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1624)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1626)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1631)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1632)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1634)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1259)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1260)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1262)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1267)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1268)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1270)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1193)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Quat](../../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1194)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1196)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1201)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Quat](../../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1202)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1204)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1571)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1572)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1574)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1579)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1580)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1582)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#977)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#978)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#980)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> <[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#1001)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#1002)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/mat3.rs.html#1004)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1535)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1536)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1538)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1611)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1612)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1614)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1639)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1640)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1642)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1251)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1252)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1254)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1275)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1276)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat3a.rs.html#1278)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1180)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Quat](../../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1188)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> <[Quat](../../prelude/struct.Quat.html "struct bevy::prelude::Quat") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Multiplies a quaternion and a 3D vector, returning the rotated vector.

##### Panics

Will panic if `self` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1181)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1209)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Quat](../../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1210)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1212)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#525)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Isometry3d](../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#526)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#529)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> <[Isometry3d](../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#338)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [GlobalTransform](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#339)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#342)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, value: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> <[GlobalTransform](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#650)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Transform](../../prelude/struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#651)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#653)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, value: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> <[Transform](../../prelude/struct.Transform.html "struct bevy::prelude::Transform") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1559)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1560)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1562)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1587)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1588)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1590)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1543)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1545)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1552)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1554)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1604)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1606)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1595)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1597)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2121)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2122)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2124)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2133)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2134)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2136)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#283)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#285)

#### fn [norm](../trait.NormedVectorSpace.html#tymethod.norm)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The size of this element. The return value should always be nonnegative.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#290)

#### fn [norm\_squared](../trait.NormedVectorSpace.html#method.norm_squared)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The squared norm of this element. Computing this is often faster than computing [`NormedVectorSpace::norm`](../trait.NormedVectorSpace.html#tymethod.norm "method bevy::math::NormedVectorSpace::norm").

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#259)

#### fn [distance](../trait.NormedVectorSpace.html#method.distance)(self, rhs: Self) -> Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The distance between this element and another, as determined by the norm.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#266)

#### fn [distance\_squared](../trait.NormedVectorSpace.html#method.distance_squared)(self, rhs: Self) -> Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The squared distance between this element and another, as determined by the norm. Note that this is often faster to compute in practice than [`NormedVectorSpace::distance`](../trait.NormedVectorSpace.html#method.distance "method bevy::math::NormedVectorSpace::distance").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#20)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#20)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [get\_represented\_type\_info](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [try\_apply](../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [reflect\_kind](../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [reflect\_ref](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [reflect\_owned](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>) -> [ReflectOwned](../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [try\_into\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [try\_as\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [try\_as\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [into\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [as\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [as\_partial\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#259)

#### fn [reflect\_partial\_eq](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [reflect\_partial\_cmp](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#259)

#### fn [debug](../../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#259)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](../../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#21)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2101)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2103-2105)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2111)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2113-2115)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [ReadFrom](../../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [AsMutVectorParts](../../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](../../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

#### fn [read\_from](../../render/render_resource/encase/internal/trait.ReadFrom.html#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](../../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](../../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [into\_any](../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [as\_any](../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [as\_any\_mut](../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [into\_reflect](../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [as\_reflect](../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [as\_reflect\_mut](../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [set](../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1927)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1928)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1930)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1939)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1940)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1942)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1947)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1948)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1950)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2043)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2044)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2046)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2051)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2052)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2054)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1991)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1992)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1994)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1999)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2000)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2002)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1955)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1956)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1958)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2031)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2032)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2034)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2059)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2060)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2062)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1979)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1980)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1982)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2007)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2008)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2010)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1963)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1965)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1972)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1974)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2024)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2026)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2015)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2017)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#639)

### impl [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#639)

#### type [Sampler](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html#associatedtype.Sampler) = UniformVec3<[UniformFloat](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/float/struct.UniformFloat.html "struct rand::distr::uniform::float::UniformFloat")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>>

The `UniformSampler` implementation supporting type `X`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Serialize as a sequence of 3 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [ShaderSize](../../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](../../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#232)

#### const [SHADER\_SIZE](../../render/render_resource/trait.ShaderSize.html#associatedconstant.SHADER_SIZE): [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> = \_

Represents [WGSL Size](https://gpuweb.github.io/gpuweb/wgsl/#alignment-and-size) (equivalent to [`ShaderType::min_size`](../../render/render_resource/trait.ShaderType.html#method.min_size "associated function bevy::render::render_resource::ShaderType::min_size"))

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](../../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#94)

#### fn [min\_size](../../render/render_resource/trait.ShaderType.html#method.min_size)() -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Represents the minimum size of `Self` (equivalent to [GPUBufferBindingLayout.minBindingSize](https://gpuweb.github.io/gpuweb/#dom-gpubufferbindinglayout-minbindingsize)) [Read more](../../render/render_resource/trait.ShaderType.html#method.min_size)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#103)

#### fn [size](../../render/render_resource/trait.ShaderType.html#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns the size of `Self` at runtime [Read more](../../render/render_resource/trait.ShaderType.html#method.size)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#206)

#### fn [assert\_uniform\_compat](../../render/render_resource/trait.ShaderType.html#method.assert_uniform_compat)()

Asserts that `Self` meets the requirements of the [uniform address space restrictions on stored values](https://gpuweb.github.io/gpuweb/wgsl/#address-spaces-uniform) and the [uniform address space layout constraints](https://gpuweb.github.io/gpuweb/wgsl/#address-space-layout-constraints) [Read more](../../render/render_resource/trait.ShaderType.html#method.assert_uniform_compat)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [field](../../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [field\_mut](../../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [field\_at](../../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [field\_at\_mut](../../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [name\_at](../../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [index\_of\_name](../../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [field\_len](../../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [iter\_fields](../../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [to\_dynamic\_struct](../../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#20)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1787)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1788)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1790)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1799)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1800)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1802)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1807)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1808)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1810)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1903)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1904)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1906)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1911)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1912)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1914)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1851)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1852)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1854)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1859)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1860)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1862)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1815)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1816)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1818)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1891)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1892)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1894)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1919)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1920)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1922)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1839)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1840)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1842)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1867)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1868)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1870)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1823)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1825)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1832)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1834)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1884)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1886)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1875)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#1877)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2081)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2083-2085)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2091)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#2093-2095)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#704)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Dir3](../../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#705)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [InvalidDirectionError](../enum.InvalidDirectionError.html "enum bevy::math::InvalidDirectionError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#707)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Dir3](../../prelude/struct.Dir3.html "struct bevy::prelude::Dir3"), <[Dir3](../../prelude/struct.Dir3.html "struct bevy::prelude::Dir3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [type\_path](../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [short\_type\_path](../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [type\_ident](../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [crate\_name](../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [module\_path](../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### fn [type\_info](../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#6)

#### type [Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#8)

#### type [Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4) = [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#11)

#### fn [xx](../../prelude/trait.Vec3Swizzles.html#tymethod.xx)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#19)

#### fn [xy](../../prelude/trait.Vec3Swizzles.html#tymethod.xy)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#27)

#### fn [with\_xy](../../prelude/trait.Vec3Swizzles.html#tymethod.with_xy)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#32)

#### fn [xz](../../prelude/trait.Vec3Swizzles.html#tymethod.xz)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#40)

#### fn [with\_xz](../../prelude/trait.Vec3Swizzles.html#tymethod.with_xz)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#45)

#### fn [yx](../../prelude/trait.Vec3Swizzles.html#tymethod.yx)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#53)

#### fn [with\_yx](../../prelude/trait.Vec3Swizzles.html#tymethod.with_yx)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#58)

#### fn [yy](../../prelude/trait.Vec3Swizzles.html#tymethod.yy)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#66)

#### fn [yz](../../prelude/trait.Vec3Swizzles.html#tymethod.yz)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#74)

#### fn [with\_yz](../../prelude/trait.Vec3Swizzles.html#tymethod.with_yz)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#79)

#### fn [zx](../../prelude/trait.Vec3Swizzles.html#tymethod.zx)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#87)

#### fn [with\_zx](../../prelude/trait.Vec3Swizzles.html#tymethod.with_zx)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#92)

#### fn [zy](../../prelude/trait.Vec3Swizzles.html#tymethod.zy)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#100)

#### fn [with\_zy](../../prelude/trait.Vec3Swizzles.html#tymethod.with_zy)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#105)

#### fn [zz](../../prelude/trait.Vec3Swizzles.html#tymethod.zz)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#113)

#### fn [xxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#118)

#### fn [xxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#123)

#### fn [xxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxz)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#128)

#### fn [xyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#133)

#### fn [xyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#138)

#### fn [xzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#143)

#### fn [xzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#148)

#### fn [xzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzz)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#153)

#### fn [yxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#158)

#### fn [yxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#163)

#### fn [yxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxz)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#168)

#### fn [yyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#173)

#### fn [yyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#178)

#### fn [yyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyz)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#183)

#### fn [yzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#188)

#### fn [yzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#193)

#### fn [yzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzz)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#198)

#### fn [zxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#203)

#### fn [zxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#208)

#### fn [zxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxz)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#213)

#### fn [zyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#218)

#### fn [zyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#223)

#### fn [zyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyz)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#228)

#### fn [zzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#233)

#### fn [zzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#238)

#### fn [zzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzz)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#243)

#### fn [xxxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#248)

#### fn [xxxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#253)

#### fn [xxxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxxz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#258)

#### fn [xxyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#263)

#### fn [xxyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#268)

#### fn [xxyz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxyz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#273)

#### fn [xxzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxzx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#278)

#### fn [xxzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxzy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#283)

#### fn [xxzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxzz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#288)

#### fn [xyxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#293)

#### fn [xyxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#298)

#### fn [xyxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xyxz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#303)

#### fn [xyyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#308)

#### fn [xyyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#313)

#### fn [xyyz](../../prelude/trait.Vec3Swizzles.html#tymethod.xyyz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#318)

#### fn [xyzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyzx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#323)

#### fn [xyzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyzy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#328)

#### fn [xyzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xyzz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#333)

#### fn [xzxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#338)

#### fn [xzxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#343)

#### fn [xzxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzxz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#348)

#### fn [xzyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#353)

#### fn [xzyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#358)

#### fn [xzyz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzyz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#363)

#### fn [xzzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzzx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#368)

#### fn [xzzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzzy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#373)

#### fn [xzzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzzz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#378)

#### fn [yxxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#383)

#### fn [yxxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#388)

#### fn [yxxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxxz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#393)

#### fn [yxyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#398)

#### fn [yxyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#403)

#### fn [yxyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxyz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#408)

#### fn [yxzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxzx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#413)

#### fn [yxzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxzy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#418)

#### fn [yxzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxzz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#423)

#### fn [yyxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#428)

#### fn [yyxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#433)

#### fn [yyxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyxz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#438)

#### fn [yyyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#443)

#### fn [yyyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#448)

#### fn [yyyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyyz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#453)

#### fn [yyzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyzx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#458)

#### fn [yyzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyzy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#463)

#### fn [yyzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyzz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#468)

#### fn [yzxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#473)

#### fn [yzxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#478)

#### fn [yzxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzxz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#483)

#### fn [yzyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#488)

#### fn [yzyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#493)

#### fn [yzyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzyz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#498)

#### fn [yzzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzzx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#503)

#### fn [yzzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzzy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#508)

#### fn [yzzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzzz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#513)

#### fn [zxxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#518)

#### fn [zxxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#523)

#### fn [zxxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxxz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#528)

#### fn [zxyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#533)

#### fn [zxyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#538)

#### fn [zxyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxyz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#543)

#### fn [zxzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxzx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#548)

#### fn [zxzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxzy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#553)

#### fn [zxzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxzz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#558)

#### fn [zyxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#563)

#### fn [zyxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#568)

#### fn [zyxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyxz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#573)

#### fn [zyyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#578)

#### fn [zyyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#583)

#### fn [zyyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyyz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#588)

#### fn [zyzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyzx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#593)

#### fn [zyzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyzy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#598)

#### fn [zyzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyzz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#603)

#### fn [zzxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#608)

#### fn [zzxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#613)

#### fn [zzxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzxz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#618)

#### fn [zzyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#623)

#### fn [zzyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#628)

#### fn [zzyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzyz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#633)

#### fn [zzzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzzx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#638)

#### fn [zzzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzzy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#643)

#### fn [zzzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzzz)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#103)

#### fn [xyz](../../prelude/trait.Vec3Swizzles.html#method.xyz)(self) -> Self

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#65)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#67)

#### const [ZERO](../trait.VectorSpace.html#associatedconstant.ZERO): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3") = Vec3::ZERO

The zero vector, which is the identity of addition for the vector space type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#66)

#### type [Scalar](../trait.VectorSpace.html#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The scalar type of this vector space.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#55)

#### fn [lerp](../trait.VectorSpace.html#method.lerp)(self, rhs: Self, t: Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")) -> Self

Perform vector space linear interpolation between this element and another, based on the parameter `t`. When `t` is `0`, `self` is recovered. When `t` is `1`, `rhs` is recovered. [Read more](../trait.VectorSpace.html#method.lerp)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [WriteInto](../../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [AsRefVectorParts](../../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](../../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

#### fn [write\_into](../../render/render_resource/encase/internal/trait.WriteInto.html#tymethod.write_into)<B>(&self, writer: &mut [Writer](../../render/render_resource/encase/internal/struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](../../render/render_resource/encase/internal/trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#21)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#98)

### impl<V> [Ease](../../prelude/trait.Ease.html "trait bevy::prelude::Ease") for V

where V: [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#99)

#### fn [interpolating\_curve\_unbounded](../../prelude/trait.Ease.html#tymethod.interpolating_curve_unbounded)(start: V, end: V) -> impl [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<V>

Given `start` and `end` values, produce a curve with [unlimited domain](../../prelude/struct.Interval.html#associatedconstant.EVERYWHERE "associated constant bevy::prelude::Interval::EVERYWHERE") that: [Read more](../../prelude/trait.Ease.html#tymethod.interpolating_curve_unbounded)

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#632)

### impl<V> [HasTangent](../trait.HasTangent.html "trait bevy::math::HasTangent") for V

where V: [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#633)

#### type [Tangent](../trait.HasTangent.html#associatedtype.Tangent) = V

The tangent type.

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#475-477)

### impl<V> [StableInterpolate](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for V

where V: [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#480)

#### fn [interpolate\_stable](../../prelude/trait.StableInterpolate.html#tymethod.interpolate_stable)(&self, other: [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> V

Interpolate between this value and the `other` given value using the parameter `t`. At `t = 0.0`, a value equivalent to `self` is recovered, while `t = 1.0` recovers a value equivalent to `other`, with intermediate values interpolating between the two. See the [trait-level documentation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for details.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#438)

#### fn [interpolate\_stable\_assign](../../prelude/trait.StableInterpolate.html#method.interpolate_stable_assign)(&mut self, other: &Self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

A version of [`interpolate_stable`](../../prelude/trait.StableInterpolate.html#tymethod.interpolate_stable "method bevy::prelude::StableInterpolate::interpolate_stable") that assigns the result to `self` for convenience.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#467)

#### fn [smooth\_nudge](../../prelude/trait.StableInterpolate.html#method.smooth_nudge)(&mut self, target: &Self, decay\_rate: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), delta: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Smoothly nudge this value towards the `target` at a given decay rate. The `decay_rate` parameter controls how fast the distance between `self` and `target` decays relative to the units of `delta`; the intended usage is for `decay_rate` to generally remain fixed, while `delta` is something like `delta_time` from an updating system. This produces a smooth following of the target that is independent of framerate. [Read more](../../prelude/trait.StableInterpolate.html#method.smooth_nudge)

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#586)

### impl<T> [TryStableInterpolate](../trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate") for T

where T: [StableInterpolate](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#587)

#### type [Error](../trait.TryStableInterpolate.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

Error produced when the value cannot be interpolated.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#588)

#### fn [try\_interpolate\_stable](../trait.TryStableInterpolate.html#tymethod.try_interpolate_stable)( &self, other: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryStableInterpolate](../trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate")\>::[Error](../trait.TryStableInterpolate.html#associatedtype.Error "type bevy::math::TryStableInterpolate::Error")\>

Attempt to interpolate the value. This may fail if the two interpolation values have different units, or if the type is not interpolable.

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Iter<Self, R, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html\\" title=\\"struct rand::distr::distribution::Iter\\">Iter</a>&lt;D, R, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;D, R, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html\\" title=\\"struct rand::distr::distribution::Iter\\">Iter</a>&lt;D, R, T&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html\\" title=\\"trait rand::distr::distribution::Distribution\\">Distribution</a>&lt;T&gt;,\\n R: <a class=\\"trait\\" href=\\"https://docs.rs/rand\_core/0.9.5/x86\_64-unknown-linux-gnu/rand\_core/trait.Rng.html\\" title=\\"trait rand\_core::Rng\\">Rng</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}