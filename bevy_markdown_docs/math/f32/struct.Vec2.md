[bevy](../../index.html)::[math](../index.html)::[f32](index.html)

# Struct Vec2 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#29)

```rust
#[repr(C)]pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
```

A 2-dimensional vector.

## Fields

`x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)``y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#34)

### impl [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#36)

#### pub const [ZERO](#associatedconstant.ZERO): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

All zeroes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#39)

#### pub const [ONE](#associatedconstant.ONE): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

All ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#42)

#### pub const [NEG\_ONE](#associatedconstant.NEG_ONE): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

All negative ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#45)

#### pub const [MIN](#associatedconstant.MIN): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

All `f32::MIN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#48)

#### pub const [MAX](#associatedconstant.MAX): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

All `f32::MAX`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#51)

#### pub const [NAN](#associatedconstant.NAN): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

All `f32::NAN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#54)

#### pub const [INFINITY](#associatedconstant.INFINITY): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

All `f32::INFINITY`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#57)

#### pub const [NEG\_INFINITY](#associatedconstant.NEG_INFINITY): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

All `f32::NEG_INFINITY`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#60)

#### pub const [X](#associatedconstant.X): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

A unit vector pointing along the positive X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#63)

#### pub const [Y](#associatedconstant.Y): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

A unit vector pointing along the positive Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#66)

#### pub const [NEG\_X](#associatedconstant.NEG_X): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

A unit vector pointing along the negative X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#69)

#### pub const [NEG\_Y](#associatedconstant.NEG_Y): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

A unit vector pointing along the negative Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#72)

#### pub const [AXES](#associatedconstant.AXES): \[[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The unit axes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#75)

#### pub const [USES\_CORE\_SIMD](#associatedconstant.USES_CORE_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec2 uses Rust Portable SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#77)

#### pub const [USES\_NEON](#associatedconstant.USES_NEON): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec2 uses Arm NEON

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#79)

#### pub const [USES\_SCALAR\_MATH](#associatedconstant.USES_SCALAR_MATH): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

Vec2 uses scalar math

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#81)

#### pub const [USES\_SSE2](#associatedconstant.USES_SSE2): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec2 uses Intel SSE2

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#83)

#### pub const [USES\_WASM\_SIMD](#associatedconstant.USES_WASM_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec2 uses WebAssembly 128-bit SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#85)

#### pub const [USES\_WASM32\_SIMD](#associatedconstant.USES_WASM32_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

👎Deprecated since 0.31.0:

Renamed to USES\_WASM\_SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#90)

#### pub const fn [new](#method.new)(x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Creates a new vector.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/2d/rotation.rs ([line 5](../../../src/rotation/rotation.rs.html#5))

```rust
5const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
```

Hide additional examples

examples/animation/easing\_functions.rs ([line 18](../../../src/easing_functions/easing_functions.rs.html#18))

```rust
18const EXTENT: Vec2 = Vec2::new(1172.0, 520.0);
19const PLOT_SIZE: Vec2 = Vec2::splat(80.0);
20
21fn setup(mut commands: Commands) {
22    commands.spawn(Camera2d);
23
24    let text_font = TextFont {
25        font_size: FontSize::Px(10.0),
26        ..default()
27    };
28
29    let chunks = [
30        // "In" row
31        EaseFunction::SineIn,
32        EaseFunction::QuadraticIn,
33        EaseFunction::CubicIn,
34        EaseFunction::QuarticIn,
35        EaseFunction::QuinticIn,
36        EaseFunction::SmoothStepIn,
37        EaseFunction::SmootherStepIn,
38        EaseFunction::CircularIn,
39        EaseFunction::ExponentialIn,
40        EaseFunction::ElasticIn,
41        EaseFunction::BackIn,
42        EaseFunction::BounceIn,
43        // "Out" row
44        EaseFunction::SineOut,
45        EaseFunction::QuadraticOut,
46        EaseFunction::CubicOut,
47        EaseFunction::QuarticOut,
48        EaseFunction::QuinticOut,
49        EaseFunction::SmoothStepOut,
50        EaseFunction::SmootherStepOut,
51        EaseFunction::CircularOut,
52        EaseFunction::ExponentialOut,
53        EaseFunction::ElasticOut,
54        EaseFunction::BackOut,
55        EaseFunction::BounceOut,
56        // "InOut" row
57        EaseFunction::SineInOut,
58        EaseFunction::QuadraticInOut,
59        EaseFunction::CubicInOut,
60        EaseFunction::QuarticInOut,
61        EaseFunction::QuinticInOut,
62        EaseFunction::SmoothStep,
63        EaseFunction::SmootherStep,
64        EaseFunction::CircularInOut,
65        EaseFunction::ExponentialInOut,
66        EaseFunction::ElasticInOut,
67        EaseFunction::BackInOut,
68        EaseFunction::BounceInOut,
69        // "Other" row
70        EaseFunction::Linear,
71        EaseFunction::Steps(4, JumpAt::End),
72        EaseFunction::Steps(4, JumpAt::Start),
73        EaseFunction::Steps(4, JumpAt::Both),
74        EaseFunction::Steps(4, JumpAt::None),
75        EaseFunction::Elastic(50.0),
76    ]
77    .chunks(COLS);
78
79    let max_rows = chunks.clone().count();
80
81    let half_extent = EXTENT / 2.;
82    let half_size = PLOT_SIZE / 2.;
83
84    for (row, functions) in chunks.enumerate() {
85        for (col, function) in functions.iter().enumerate() {
86            let color = Hsla::hsl(col as f32 / COLS as f32 * 360.0, 0.8, 0.75).into();
87            commands.spawn((
88                EaseFunctionPlot(*function, color),
89                Transform::from_xyz(
90                    -half_extent.x + EXTENT.x / (COLS - 1) as f32 * col as f32,
91                    half_extent.y - EXTENT.y / (max_rows - 1) as f32 * row as f32,
92                    0.0,
93                ),
94                children![
95                    (
96                        Sprite::from_color(color, Vec2::splat(5.0)),
97                        Transform::from_xyz(half_size.x + 5.0, -half_size.y, 0.0),
98                    ),
99                    (
100                        Sprite::from_color(color, Vec2::splat(4.0)),
101                        Transform::from_xyz(-half_size.x, -half_size.y, 0.0),
102                    ),
103                    (
104                        Text2d(format!("{function:?}")),
105                        text_font.clone(),
106                        TextColor(color),
107                        Transform::from_xyz(0.0, -half_size.y - 15.0, 0.0),
108                    )
109                ],
110            ));
111        }
112    }
113    commands.spawn((
114        Text::default(),
115        Node {
116            position_type: PositionType::Absolute,
117            top: px(12),
118            left: px(12),
119            ..default()
120        },
121    ));
122}
123
124fn display_curves(
125    mut gizmos: Gizmos,
126    ease_functions: Query<(&EaseFunctionPlot, &Transform, &Children)>,
127    mut transforms: Query<&mut Transform, Without<EaseFunctionPlot>>,
128    mut ui_text: Single<&mut Text>,
129    time: Res<Time>,
130) {
131    let samples = 100;
132    let duration = 2.5;
133    let time_margin = 0.5;
134
135    let now = ((time.elapsed_secs() % (duration + time_margin * 2.0) - time_margin) / duration)
136        .clamp(0.0, 1.0);
137
138    ui_text.0 = format!("Progress: {now:.2}");
139
140    for (EaseFunctionPlot(function, color), transform, children) in &ease_functions {
141        let center = transform.translation.xy();
142        let half_size = PLOT_SIZE / 2.0;
143
144        // Draw a box around the curve
145        gizmos.linestrip_2d(
146            [
147                center + half_size,
148                center + half_size * Vec2::new(-1., 1.),
149                center + half_size * Vec2::new(-1., -1.),
150                center + half_size * Vec2::new(1., -1.),
151                center + half_size,
152            ],
153            color.darker(0.4),
154        );
155
156        // Draw the curve
157        let f = EasingCurve::new(0.0, 1.0, *function);
158        let drawn_curve = f
159            .by_ref()
160            .graph()
161            .map(|(x, y)| center - half_size + Vec2::new(x, y) * PLOT_SIZE);
162        gizmos.curve_2d(
163            &drawn_curve,
164            drawn_curve.domain().spaced_points(samples).unwrap(),
165            *color,
166        );
167
168        // Show progress along the curve for the current time
169        let y = f.sample(now).unwrap() * PLOT_SIZE.y;
170        transforms.get_mut(children[0]).unwrap().translation.y = -half_size.y + y;
171        transforms.get_mut(children[1]).unwrap().translation =
172            -half_size.extend(0.0) + Vec3::new(now * PLOT_SIZE.x, y, 0.0);
173
174        // Show horizontal bar at y value
175        gizmos.linestrip_2d(
176            [
177                center - half_size + Vec2::Y * y,
178                center - half_size + Vec2::new(PLOT_SIZE.x, y),
179            ],
180            color.darker(0.2),
181        );
182    }
183}
```

examples/showcase/breakout.rs ([line 14](../../../src/breakout/breakout.rs.html#14))

```rust
14const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
15const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
16const PADDLE_SPEED: f32 = 500.0;
17// How close can the paddle get to the wall
18const PADDLE_PADDING: f32 = 10.0;
19
20// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
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

examples/math/render\_primitives.rs ([line 154](../../../src/render_primitives/render_primitives.rs.html#154))

```rust
153const RECTANGLE: Rectangle = Rectangle {
154    half_size: Vec2::new(SMALL_2D, BIG_2D),
155};
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
```

examples/animation/color\_animation.rs ([line 75](../../../src/color_animation/color_animation.rs.html#75))

```rust
73fn spawn_curve_sprite<T: CurveColor>(commands: &mut Commands, y: f32, points: [T; 4]) {
74    commands.spawn((
75        Sprite::sized(Vec2::new(75., 75.)),
76        Transform::from_xyz(0., y, 0.),
77        Curve(CubicBezier::new([points]).to_curve().unwrap()),
78    ));
79}
80
81fn spawn_mixed_sprite<T: MixedColor>(commands: &mut Commands, y: f32, colors: [T; 4]) {
82    commands.spawn((
83        Transform::from_xyz(0., y, 0.),
84        Sprite::sized(Vec2::new(75., 75.)),
85        Mixed(colors),
86    ));
87}
```

examples/ecs/observers.rs ([lines 74-77](../../../src/observers/observers.rs.html#74-77))

```rust
72    fn random(rand: &mut ChaCha8Rng) -> Self {
73        Mine {
74            pos: Vec2::new(
75                (rand.random::<f32>() - 0.5) * 1200.0,
76                (rand.random::<f32>() - 0.5) * 600.0,
77            ),
78            size: 4.0 + rand.random::<f32>() * 16.0,
79        }
80    }
```

Additional examples can be found in:  

*   [examples/math/custom\_primitives.rs](../../../src/custom_primitives/custom_primitives.rs.html#49)
*   [examples/window/window\_resizing.rs](../../../src/window_resizing/window_resizing.rs.html#7)
*   [examples/3d/motion\_blur.rs](../../../src/motion_blur/motion_blur.rs.html#297)
*   [tests/window/minimizing.rs](../../../src/minimizing/minimizing.rs.html#75)
*   [tests/window/resizing.rs](../../../src/resizing/resizing.rs.html#151)
*   [examples/camera/first\_person\_view\_model.rs](../../../src/first_person_view_model/first_person_view_model.rs.html#82)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#286)
*   [examples/ecs/parallel\_query.rs](../../../src/parallel_query/parallel_query.rs.html#22)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../../src/scroll/scroll.rs.html#34)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#123)
*   [examples/remote/server.rs](../../../src/server/server.rs.html#48)
*   [examples/ecs/fallible\_params.rs](../../../src/fallible_params/fallible_params.rs.html#98-101)
*   [examples/gizmos/2d\_text\_gizmos.rs](../../../src/2d_text_gizmos/2d_text_gizmos.rs.html#52-55)
*   [examples/2d/wireframe\_2d.rs](../../../src/wireframe_2d/wireframe_2d.rs.html#59)
*   [examples/stress\_tests/many\_sprites.rs](../../../src/many_sprites/many_sprites.rs.html#75)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../../src/many_sprite_meshes/many_sprite_meshes.rs.html#77)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#131)
*   [examples/math/bounding\_2d.rs](../../../src/bounding_2d/bounding_2d.rs.html#223)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../../src/many_animated_sprites/many_animated_sprites.rs.html#73)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#75)
*   [examples/stress\_tests/many\_text2d.rs](../../../src/many_text2d/many_text2d.rs.html#126)
*   [examples/camera/2d\_on\_ui.rs](../../../src/2d_on_ui/2d_on_ui.rs.html#59)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#480)
*   [examples/2d/cpu\_draw.rs](../../../src/cpu_draw/cpu_draw.rs.html#61)
*   [examples/gizmos/2d\_gizmos.rs](../../../src/2d_gizmos/2d_gizmos.rs.html#54)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#862)
*   [examples/2d/sprite\_slice.rs](../../../src/sprite_slice/sprite_slice.rs.html#32)
*   [examples/asset/repeated\_texture.rs](../../../src/repeated_texture/repeated_texture.rs.html#60)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#71)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#107)
*   [examples/3d/auto\_exposure.rs](../../../src/auto_exposure/auto_exposure.rs.html#71)
*   [examples/stress\_tests/many\_cubes.rs](../../../src/many_cubes/many_cubes.rs.html#459)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../../src/scrollbars/scrollbars.rs.html#78)
*   [examples/2d/2d\_shapes.rs](../../../src/2d_shapes/2d_shapes.rs.html#66)
*   [examples/2d/sprite\_scale.rs](../../../src/sprite_scale/sprite_scale.rs.html#23)
*   [examples/2d/text2d.rs](../../../src/text2d/text2d.rs.html#79)
*   [examples/3d/3d\_shapes.rs](../../../src/3d_shapes/3d_shapes.rs.html#96)
*   [examples/3d/camera\_sub\_view.rs](../../../src/camera_sub_view/camera_sub_view.rs.html#83)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#97)

#### pub const fn [splat](#method.splat)(v: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Creates a vector with all elements set to `v`.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/animation/easing\_functions.rs ([line 19](../../../src/easing_functions/easing_functions.rs.html#19))

```rust
19const PLOT_SIZE: Vec2 = Vec2::splat(80.0);
20
21fn setup(mut commands: Commands) {
22    commands.spawn(Camera2d);
23
24    let text_font = TextFont {
25        font_size: FontSize::Px(10.0),
26        ..default()
27    };
28
29    let chunks = [
30        // "In" row
31        EaseFunction::SineIn,
32        EaseFunction::QuadraticIn,
33        EaseFunction::CubicIn,
34        EaseFunction::QuarticIn,
35        EaseFunction::QuinticIn,
36        EaseFunction::SmoothStepIn,
37        EaseFunction::SmootherStepIn,
38        EaseFunction::CircularIn,
39        EaseFunction::ExponentialIn,
40        EaseFunction::ElasticIn,
41        EaseFunction::BackIn,
42        EaseFunction::BounceIn,
43        // "Out" row
44        EaseFunction::SineOut,
45        EaseFunction::QuadraticOut,
46        EaseFunction::CubicOut,
47        EaseFunction::QuarticOut,
48        EaseFunction::QuinticOut,
49        EaseFunction::SmoothStepOut,
50        EaseFunction::SmootherStepOut,
51        EaseFunction::CircularOut,
52        EaseFunction::ExponentialOut,
53        EaseFunction::ElasticOut,
54        EaseFunction::BackOut,
55        EaseFunction::BounceOut,
56        // "InOut" row
57        EaseFunction::SineInOut,
58        EaseFunction::QuadraticInOut,
59        EaseFunction::CubicInOut,
60        EaseFunction::QuarticInOut,
61        EaseFunction::QuinticInOut,
62        EaseFunction::SmoothStep,
63        EaseFunction::SmootherStep,
64        EaseFunction::CircularInOut,
65        EaseFunction::ExponentialInOut,
66        EaseFunction::ElasticInOut,
67        EaseFunction::BackInOut,
68        EaseFunction::BounceInOut,
69        // "Other" row
70        EaseFunction::Linear,
71        EaseFunction::Steps(4, JumpAt::End),
72        EaseFunction::Steps(4, JumpAt::Start),
73        EaseFunction::Steps(4, JumpAt::Both),
74        EaseFunction::Steps(4, JumpAt::None),
75        EaseFunction::Elastic(50.0),
76    ]
77    .chunks(COLS);
78
79    let max_rows = chunks.clone().count();
80
81    let half_extent = EXTENT / 2.;
82    let half_size = PLOT_SIZE / 2.;
83
84    for (row, functions) in chunks.enumerate() {
85        for (col, function) in functions.iter().enumerate() {
86            let color = Hsla::hsl(col as f32 / COLS as f32 * 360.0, 0.8, 0.75).into();
87            commands.spawn((
88                EaseFunctionPlot(*function, color),
89                Transform::from_xyz(
90                    -half_extent.x + EXTENT.x / (COLS - 1) as f32 * col as f32,
91                    half_extent.y - EXTENT.y / (max_rows - 1) as f32 * row as f32,
92                    0.0,
93                ),
94                children![
95                    (
96                        Sprite::from_color(color, Vec2::splat(5.0)),
97                        Transform::from_xyz(half_size.x + 5.0, -half_size.y, 0.0),
98                    ),
99                    (
100                        Sprite::from_color(color, Vec2::splat(4.0)),
101                        Transform::from_xyz(-half_size.x, -half_size.y, 0.0),
102                    ),
103                    (
104                        Text2d(format!("{function:?}")),
105                        text_font.clone(),
106                        TextColor(color),
107                        Transform::from_xyz(0.0, -half_size.y - 15.0, 0.0),
108                    )
109                ],
110            ));
111        }
112    }
113    commands.spawn((
114        Text::default(),
115        Node {
116            position_type: PositionType::Absolute,
117            top: px(12),
118            left: px(12),
119            ..default()
120        },
121    ));
122}
```

Hide additional examples

examples/ecs/delayed\_commands.rs ([line 19](../../../src/delayed_commands/delayed_commands.rs.html#19))

```rust
19const SQUARE_SIZE: Vec2 = Vec2::splat(45.0);
```

examples/2d/sprite\_tile.rs ([line 47](../../../src/sprite_tile/sprite_tile.rs.html#47))

```rust
41fn animate(mut sprites: Query<&mut Sprite>, mut state: ResMut<AnimationState>, time: Res<Time>) {
42    if state.current >= state.max || state.current <= state.min {
43        state.speed = -state.speed;
44    };
45    state.current += state.speed * time.delta_secs();
46    for mut sprite in &mut sprites {
47        sprite.custom_size = Some(Vec2::splat(state.current));
48    }
49}
```

examples/3d/mixed\_lighting.rs ([line 89](../../../src/mixed_lighting/mixed_lighting.rs.html#89))

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
```

examples/3d/ssr.rs ([line 330](../../../src/ssr/ssr.rs.html#330))

```rust
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

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 223](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#223))

```rust
217fn spawn_plane(
218    commands: &mut Commands,
219    meshes: &mut Assets<Mesh>,
220    show_depth_texture_materials: &mut Assets<ShowDepthTextureMaterial>,
221    demo_depth_texture: &DemoDepthTexture,
222) {
223    let plane_handle = meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(2.0)));
224    let show_depth_texture_material = show_depth_texture_materials.add(ShowDepthTextureMaterial {
225        depth_texture: Some(demo_depth_texture.0.clone()),
226    });
227    commands.spawn((
228        Mesh3d(plane_handle),
229        MeshMaterial3d(show_depth_texture_material),
230        Transform::from_xyz(10.0, 4.0, 0.0).with_scale(Vec3::splat(2.5)),
231    ));
232}
```

Additional examples can be found in:  

*   [examples/gizmos/anchored\_text\_gizmos.rs](../../../src/anchored_text_gizmos/anchored_text_gizmos.rs.html#30)
*   [examples/ecs/parallel\_query.rs](../../../src/parallel_query/parallel_query.rs.html#21)
*   [examples/math/bounding\_2d.rs](../../../src/bounding_2d/bounding_2d.rs.html#329)
*   [examples/camera/first\_person\_view\_model.rs](../../../src/first_person_view_model/first_person_view_model.rs.html#155)
*   [examples/3d/clustered\_decals.rs](../../../src/clustered_decals/clustered_decals.rs.html#228)
*   [examples/math/custom\_primitives.rs](../../../src/custom_primitives/custom_primitives.rs.html#441)
*   [examples/3d/clustered\_decal\_maps.rs](../../../src/clustered_decal_maps/clustered_decal_maps.rs.html#197)
*   [examples/ui/ui\_transform.rs](../../../src/ui_transform/ui_transform.rs.html#59)
*   [examples/stress\_tests/many\_sprites.rs](../../../src/many_sprites/many_sprites.rs.html#59)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../../src/many_sprite_meshes/many_sprite_meshes.rs.html#61)
*   [examples/audio/spatial\_audio\_2d.rs](../../../src/spatial_audio_2d/spatial_audio_2d.rs.html#53)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#225)
*   [examples/showcase/contributors.rs](../../../src/contributors/contributors.rs.html#124)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../../src/many_animated_sprites/many_animated_sprites.rs.html#56)
*   [examples/2d/bloom\_2d.rs](../../../src/bloom_2d/bloom_2d.rs.html#38)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#58)
*   [examples/stress\_tests/many\_text2d.rs](../../../src/many_text2d/many_text2d.rs.html#111)
*   [examples/camera/free\_camera\_controller.rs](../../../src/free_camera_controller/free_camera_controller.rs.html#245)
*   [examples/3d/atmosphere.rs](../../../src/atmosphere/atmosphere.rs.html#267)
*   [examples/time/virtual\_time.rs](../../../src/virtual_time/virtual_time.rs.html#51)
*   [examples/picking/sprite\_picking.rs](../../../src/sprite_picking/sprite_picking.rs.html#35)
*   [examples/gizmos/2d\_gizmos.rs](../../../src/2d_gizmos/2d_gizmos.rs.html#47)
*   [examples/2d/mesh2d\_alpha\_mode.rs](../../../src/mesh2d_alpha_mode/mesh2d_alpha_mode.rs.html#26)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#856)
*   [examples/2d/sprite\_slice.rs](../../../src/sprite_slice/sprite_slice.rs.html#25)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#123)
*   [examples/stress\_tests/many\_cubes.rs](../../../src/many_cubes/many_cubes.rs.html#468)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#244)
*   [examples/2d/2d\_shapes.rs](../../../src/2d_shapes/2d_shapes.rs.html#109)
*   [examples/showcase/breakout.rs](../../../src/breakout/breakout.rs.html#204)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#147)
*   [examples/ui/navigation/directional\_navigation.rs](../../../src/directional_navigation/directional_navigation.rs.html#212)
*   [examples/3d/3d\_shapes.rs](../../../src/3d_shapes/3d_shapes.rs.html#120)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#104-106)

#### pub fn [map](#method.map)<F>(self, f: F) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html),

Returns a vector containing each element of `self` modified by a mapping function `f`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#118)

#### pub fn [select](#method.select)(mask: [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2"), if\_true: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), if\_false: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Creates a vector from the elements in `if_true` and `if_false`, selecting which to use for each element of `self`.

A true element in the mask uses the corresponding element from `if_true`, and false uses the element from `if_false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#128)

#### pub const fn [from\_array](#method.from_array)(a: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Creates a new vector from an array.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#135)

#### pub const fn [to\_array](#method.to_array)(&self) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts `self` to `[x, y]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#146)

#### pub const fn [from\_slice](#method.from_slice)(slice: &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Creates a vector from the first 2 values in `slice`.

##### Panics

Panics if `slice` is less than 2 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#157)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\])

Writes the elements of `self` to the first 2 elements in `slice`.

##### Panics

Panics if `slice` is less than 2 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#164)

#### pub const fn [extend](#method.extend)(self, z: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Creates a 3D vector from `self` and the given `z` value.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/3d/clustered\_decals.rs ([line 342](../../../src/clustered_decals/clustered_decals.rs.html#342))

```rust
338fn calculate_initial_decal_transform(start: Vec3, looking_at: Vec3, size: Vec2) -> Transform {
339    let direction = looking_at - start;
340    let center = start + direction * 0.5;
341    Transform::from_translation(center)
342        .with_scale((size * 0.5).extend(direction.length()))
343        .looking_to(direction, Vec3::Y)
344}
```

Hide additional examples

examples/math/cubic\_splines.rs ([line 176](../../../src/cubic_splines/cubic_splines.rs.html#176))

```rust
169fn draw_curve(curve: Res<Curve>, mut gizmos: Gizmos) {
170    let Some(ref curve) = curve.0 else {
171        return;
172    };
173    // Scale resolution with curve length so it doesn't degrade as the length increases.
174    let resolution = 100 * curve.segments().len();
175    gizmos.linestrip(
176        curve.iter_positions(resolution).map(|pt| pt.extend(0.0)),
177        Color::srgb(1.0, 1.0, 1.0),
178    );
179}
```

examples/ecs/fallible\_params.rs ([line 129](../../../src/fallible_params/fallible_params.rs.html#129))

```rust
124fn move_targets(mut enemies: Populated<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
125    for (mut transform, mut target) in &mut *enemies {
126        target.rotation += target.rotation_speed * time.delta_secs();
127        transform.rotation = Quat::from_rotation_z(target.rotation);
128        let offset = transform.right() * target.radius;
129        transform.translation = target.origin.extend(0.0) + offset;
130    }
131}
```

examples/3d/reflection\_probes.rs ([line 355](../../../src/reflection_probes/reflection_probes.rs.html#355))

```rust
343fn rotate_camera(
344    time: Res<Time>,
345    mut camera_query: Query<&mut Transform, With<Camera3d>>,
346    app_status: Res<AppStatus>,
347) {
348    if !app_status.rotating {
349        return;
350    }
351
352    for mut transform in camera_query.iter_mut() {
353        transform.translation = Vec2::from_angle(time.delta_secs() * PI / 5.0)
354            .rotate(transform.translation.xz())
355            .extend(transform.translation.y)
356            .xzy();
357        transform.look_at(Vec3::ZERO, Vec3::Y);
358    }
359}
```

examples/3d/irradiance\_volumes.rs ([line 361](../../../src/irradiance_volumes/irradiance_volumes.rs.html#361))

```rust
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
```

examples/2d/dynamic\_mip\_generation.rs ([line 438](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#438))

```rust
427fn animate_image_scale(
428    mut animated_images_query: Query<&mut Transform, With<AnimatedImage>>,
429    windows_query: Query<&Window, With<PrimaryWindow>>,
430    app_status: Res<AppStatus>,
431    time: Res<Time>,
432) {
433    let window_size = windows_query.iter().next().unwrap().size();
434    let animated_mesh_size = app_status.animated_mesh_size(window_size);
435
436    for mut animated_image_transform in &mut animated_images_query {
437        animated_image_transform.scale =
438            animated_mesh_size.extend(1.0) * triangle_wave(time.elapsed_secs(), ANIMATION_PERIOD);
439    }
440}
441
442/// Evaluates a [triangle wave] with the given wavelength.
443///
444/// This is used as part of [`animate_image_scale`], to derive the scale from
445/// the current elapsed time.
446///
447/// [triangle wave]: https://en.wikipedia.org/wiki/Triangle_wave#Definition
448fn triangle_wave(time: f32, wavelength: f32) -> f32 {
449    2.0 * ops::abs(time / wavelength - ops::floor(time / wavelength + 0.5))
450}
451
452/// Adds the top mipmap level of the image to [`MipGenerationJobs`].
453///
454/// Note that this must run in the render world, not the main world, as
455/// [`MipGenerationJobs`] is a resource that exists in the former. Consequently,
456/// it must use [`Extract`] to access main world resources.
457fn extract_mipmap_source_image(
458    mipmap_source_image: Extract<Res<MipmapSourceImage>>,
459    app_status: Extract<Res<AppStatus>>,
460    mut mip_generation_jobs: ResMut<MipGenerationJobs>,
461) {
462    if app_status.enable_mip_generation == EnableMipGeneration::On {
463        mip_generation_jobs.add(MIP_GENERATION_PHASE_ID, mipmap_source_image.id());
464    }
465}
466
467/// Updates the widgets at the bottom of the screen to reflect the settings that
468/// the user has chosen.
469fn update_radio_buttons(
470    mut widgets: Query<
471        (
472            Entity,
473            Option<&mut BackgroundColor>,
474            Has<Text>,
475            &WidgetClickSender<AppSetting>,
476        ),
477        Or<(With<RadioButton>, With<RadioButtonText>)>,
478    >,
479    app_status: Res<AppStatus>,
480    mut writer: TextUiWriter,
481) {
482    for (entity, image, has_text, sender) in widgets.iter_mut() {
483        let selected = match **sender {
484            AppSetting::RegenerateTopMipLevel => continue,
485            AppSetting::EnableMipGeneration(enable_mip_generation) => {
486                enable_mip_generation == app_status.enable_mip_generation
487            }
488            AppSetting::ImageWidth(image_width) => image_width == app_status.image_width,
489            AppSetting::ImageHeight(image_height) => image_height == app_status.image_height,
490        };
491
492        if let Some(mut bg_color) = image {
493            widgets::update_ui_radio_button(&mut bg_color, selected);
494        }
495        if has_text {
496            widgets::update_ui_radio_button_text(entity, &mut writer, selected);
497        }
498    }
499}
500
501/// Handles a request from the user to change application settings via the UI.
502///
503/// This also handles clicks on the "Regenerate Top Mip Level" button.
504fn handle_app_setting_change(
505    mut events: MessageReader<WidgetClickEvent<AppSetting>>,
506    mut app_status: ResMut<AppStatus>,
507    mut regenerate_image_message_writer: MessageWriter<RegenerateImage>,
508) {
509    for event in events.read() {
510        // If this is a setting, update the setting. Fall through if, in
511        // addition to updating the setting, we need to regenerate the image.
512        match **event {
513            AppSetting::EnableMipGeneration(enable_mip_generation) => {
514                app_status.enable_mip_generation = enable_mip_generation;
515                continue;
516            }
517
518            AppSetting::RegenerateTopMipLevel => {}
519            AppSetting::ImageWidth(image_size) => app_status.image_width = image_size,
520            AppSetting::ImageHeight(image_size) => app_status.image_height = image_size,
521        }
522
523        // Schedule the image to be regenerated.
524        regenerate_image_message_writer.write(RegenerateImage);
525    }
526}
527
528/// Handles resize events for the window.
529///
530/// Resizing the window invalidates the image and repositions all image views.
531/// (Regenerating the image isn't strictly necessary, but it's simplest to have
532/// a single function that both regenerates the image and recreates the image
533/// views.)
534fn handle_window_resize_events(
535    mut events: MessageReader<WindowResized>,
536    mut regenerate_image_message_writer: MessageWriter<RegenerateImage>,
537) {
538    for _ in events.read() {
539        regenerate_image_message_writer.write(RegenerateImage);
540    }
541}
542
543/// Recreates the image, as well as all views that show the image, when a
544/// [`RegenerateImage`] message is received.
545///
546/// The views that show the image consist of the animated mesh on the left side
547/// of the window and the column of mipmap level views on the right side of the
548/// window.
549fn regenerate_image_when_requested(
550    mut commands: Commands,
551    image_views_query: Query<Entity, With<ImageView>>,
552    windows_query: Query<&Window, With<PrimaryWindow>>,
553    app_assets: Res<AppAssets>,
554    mut app_status: ResMut<AppStatus>,
555    mut images: ResMut<Assets<Image>>,
556    mut single_mip_level_materials: ResMut<Assets<SingleMipLevelMaterial>>,
557    mut color_materials: ResMut<Assets<ColorMaterial>>,
558    mut message_reader: MessageReader<RegenerateImage>,
559) {
560    // Only do this at most once per frame, or else the despawn logic below will
561    // get confused.
562    if message_reader.read().count() == 0 {
563        return;
564    }
565
566    // Despawn all entities that show the image.
567    for entity in image_views_query.iter() {
568        commands.entity(entity).despawn();
569    }
570
571    // Regenerate the image.
572    let image_handle = app_status.regenerate_mipmap_source_image(&mut commands, &mut images);
573
574    // Respawn the animated image view on the left side of the window.
575    spawn_animated_mesh(
576        &mut commands,
577        &app_status,
578        &app_assets,
579        &windows_query,
580        &mut color_materials,
581        &image_handle,
582    );
583
584    // Respawn the column of mip level views on the right side of the window.
585    spawn_mip_level_views(
586        &mut commands,
587        &app_status,
588        &app_assets,
589        &windows_query,
590        &mut single_mip_level_materials,
591        &image_handle,
592    );
593}
594
595/// Spawns the image on the left that continually changes scale.
596///
597/// Continually changing scale effectively cycles though each mip level,
598/// demonstrating the difference between mip level images being present and mip
599/// level image being absent.
600fn spawn_animated_mesh(
601    commands: &mut Commands,
602    app_status: &AppStatus,
603    app_assets: &AppAssets,
604    windows_query: &Query<&Window, With<PrimaryWindow>>,
605    color_materials: &mut Assets<ColorMaterial>,
606    image_handle: &Handle<Image>,
607) {
608    let window_size = windows_query.iter().next().unwrap().size();
609    let animated_mesh_area_size = app_status.animated_mesh_area_size(window_size);
610    let animated_mesh_size = app_status.animated_mesh_size(window_size);
611
612    commands.spawn((
613        Mesh2d(app_assets.rectangle.clone()),
614        MeshMaterial2d(color_materials.add(ColorMaterial {
615            texture: Some(image_handle.clone()),
616            ..default()
617        })),
618        Transform::from_translation(
619            (animated_mesh_area_size * 0.5 - window_size * 0.5).extend(0.0),
620        )
621        .with_scale(animated_mesh_size.extend(1.0)),
622        AnimatedImage,
623        ImageView,
624    ));
625}
626
627/// Creates the column on the right side of the window that displays each mip
628/// level by itself.
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

Additional examples can be found in:  

*   [examples/stress\_tests/many\_gizmos.rs](../../../src/many_gizmos/many_gizmos.rs.html#71)
*   [examples/showcase/breakout.rs](../../../src/breakout/breakout.rs.html#153)
*   [examples/ecs/parallel\_query.rs](../../../src/parallel_query/parallel_query.rs.html#21)
*   [examples/camera/2d\_top\_down\_camera.rs](../../../src/2d_top_down_camera/2d_top_down_camera.rs.html#115)
*   [examples/2d/rotation.rs](../../../src/rotation/rotation.rs.html#167)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#33)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#124)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#227)
*   [examples/stress\_tests/many\_sprites.rs](../../../src/many_sprites/many_sprites.rs.html#76)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../../src/many_sprite_meshes/many_sprite_meshes.rs.html#78)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#225)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../../src/many_animated_sprites/many_animated_sprites.rs.html#74)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#76)
*   [examples/stress\_tests/many\_text2d.rs](../../../src/many_text2d/many_text2d.rs.html#127)
*   [examples/animation/easing\_functions.rs](../../../src/easing_functions/easing_functions.rs.html#172)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#480)
*   [examples/time/virtual\_time.rs](../../../src/virtual_time/virtual_time.rs.html#51)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#126)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#147)
*   [examples/2d/text2d.rs](../../../src/text2d/text2d.rs.html#85)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#171)

#### pub fn [with\_x](#method.with_x)(self, x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Creates a 2D vector from `self` with the given value of `x`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#179)

#### pub fn [with\_y](#method.with_y)(self, y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Creates a 2D vector from `self` with the given value of `y`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#187)

#### pub fn [dot](#method.dot)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the dot product of `self` and `rhs`.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/2d/rotation.rs ([line 211](../../../src/rotation/rotation.rs.html#211))

```rust
195fn rotate_to_player_system(
196    time: Res<Time>,
197    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
198    player_transform: Single<&Transform, With<Player>>,
199) {
200    // Get the player translation in 2D
201    let player_translation = player_transform.translation.xy();
202
203    for (config, mut enemy_transform) in &mut query {
204        // Get the enemy ship forward vector in 2D (already unit length)
205        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();
206
207        // Get the vector from the enemy ship to the player ship in 2D and normalize it.
208        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
209
210        // Get the dot product between the enemy forward vector and the direction to the player.
211        let forward_dot_player = enemy_forward.dot(to_player);
212
213        // If the dot product is approximately 1.0 then the enemy is already facing the player and
214        // we can early out.
215        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
216            continue;
217        }
218
219        // Get the right vector of the enemy ship in 2D (already unit length)
220        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();
221
222        // Get the dot product of the enemy right vector and the direction to the player ship.
223        // If the dot product is negative them we need to rotate counter clockwise, if it is
224        // positive we need to rotate clockwise. Note that `copysign` will still return 1.0 if the
225        // dot product is 0.0 (because the player is directly behind the enemy, so perpendicular
226        // with the right vector).
227        let right_dot_player = enemy_right.dot(to_player);
228
229        // Determine the sign of rotation from the right dot player. We need to negate the sign
230        // here as the 2D bevy co-ordinate system rotates around +Z, which is pointing out of the
231        // screen. Due to the right hand rule, positive rotation around +Z is counter clockwise and
232        // negative is clockwise.
233        let rotation_sign = -f32::copysign(1.0, right_dot_player);
234
235        // Limit rotation so we don't overshoot the target. We need to convert our dot product to
236        // an angle here so we can get an angle of rotation to clamp against.
237        let max_angle = ops::acos(forward_dot_player.clamp(-1.0, 1.0)); // Clamp acos for safety
238
239        // Calculate angle of rotation with limit
240        let rotation_angle =
241            rotation_sign * (config.rotation_speed * time.delta_secs()).min(max_angle);
242
243        // Rotate the enemy to face the player
244        enemy_transform.rotate_z(rotation_angle);
245    }
246}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#194)

#### pub fn [dot\_into\_vec](#method.dot_into_vec)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector where every component is the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#206)

#### pub fn [min](#method.min)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the minimum values for each element of `self` and `rhs`.

In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/math/custom\_primitives.rs ([line 449](../../../src/custom_primitives/custom_primitives.rs.html#449))

```rust
435    fn aabb_2d(&self, isometry: impl Into<Isometry2d>) -> Aabb2d {
436        let isometry = isometry.into();
437
438        // The center of the circle at the center of the right wing of the heart
439        let circle_center = isometry.rotation * Vec2::new(self.radius, 0.0);
440        // The maximum X and Y positions of the two circles of the wings of the heart.
441        let max_circle = circle_center.abs() + Vec2::splat(self.radius);
442        // Since the two circles of the heart are mirrored around the origin, the minimum position is the negative of the maximum.
443        let min_circle = -max_circle;
444
445        // The position of the tip at the bottom of the heart
446        let tip_position = isometry.rotation * Vec2::new(0.0, -self.radius * (1. + SQRT_2));
447
448        Aabb2d {
449            min: isometry.translation + min_circle.min(tip_position),
450            max: isometry.translation + max_circle.max(tip_position),
451        }
452    }
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#221)

#### pub fn [max](#method.max)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the maximum values for each element of `self` and `rhs`.

In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/math/custom\_primitives.rs ([line 450](../../../src/custom_primitives/custom_primitives.rs.html#450))

```rust
435    fn aabb_2d(&self, isometry: impl Into<Isometry2d>) -> Aabb2d {
436        let isometry = isometry.into();
437
438        // The center of the circle at the center of the right wing of the heart
439        let circle_center = isometry.rotation * Vec2::new(self.radius, 0.0);
440        // The maximum X and Y positions of the two circles of the wings of the heart.
441        let max_circle = circle_center.abs() + Vec2::splat(self.radius);
442        // Since the two circles of the heart are mirrored around the origin, the minimum position is the negative of the maximum.
443        let min_circle = -max_circle;
444
445        // The position of the tip at the bottom of the heart
446        let tip_position = isometry.rotation * Vec2::new(0.0, -self.radius * (1. + SQRT_2));
447
448        Aabb2d {
449            min: isometry.translation + min_circle.min(tip_position),
450            max: isometry.translation + max_circle.max(tip_position),
451        }
452    }
```

Hide additional examples

examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs ([line 49](../../../src/drag_to_scroll/drag_to_scroll.rs.html#49))

```rust
22fn setup(mut commands: Commands) {
23    let w = 60;
24    let h = 40;
25
26    commands.spawn(Camera2d);
27    commands.insert_resource(UiScale(0.5));
28
29    commands
30        .spawn((
31            Node {
32                width: percent(100),
33                height: percent(100),
34                overflow: Overflow::scroll(),
35                ..Default::default()
36            },
37            ScrollPosition(Vec2::ZERO),
38            ScrollableNode,
39            ScrollStart(Vec2::ZERO),
40        ))
41        .observe(
42            |drag: On<Pointer<Drag>>,
43             ui_scale: Res<UiScale>,
44             mut scroll_position_query: Query<
45                (&mut ScrollPosition, &ScrollStart),
46                With<ScrollableNode>,
47            >| {
48                if let Ok((mut scroll_position, start)) = scroll_position_query.single_mut() {
49                    scroll_position.0 = (start.0 - drag.distance / ui_scale.0).max(Vec2::ZERO);
50                }
51            },
52        )
53        .observe(
54            |_: On<Pointer<DragStart>>,
55             mut scroll_position_query: Query<
56                (&ComputedNode, &mut ScrollStart),
57                With<ScrollableNode>,
58            >| {
59                if let Ok((computed_node, mut start)) = scroll_position_query.single_mut() {
60                    start.0 = computed_node.scroll_position * computed_node.inverse_scale_factor;
61                }
62            },
63        )
64        .with_children(|commands| {
65            commands
66                .spawn((
67                    Node {
68                        display: Display::Grid,
69                        grid_template_rows: RepeatedGridTrack::px(w as i32, 100.),
70                        grid_template_columns: RepeatedGridTrack::px(h as i32, 100.),
71                        ..default()
72                    },
73                    Pickable {
74                        is_hoverable: false,
75                        should_block_lower: true,
76                    }
77                ))
78                .with_children(|commands| {
79                    for y in 0..h {
80                        for x in 0..w {
81                            let tile_color = if (x + y) % 2 == 1 {
82                                let hue = ((x as f32 / w as f32) * 270.0)
83                                    + ((y as f32 / h as f32) * 90.0);
84                                Color::hsl(hue, 1., 0.5)
85                            } else {
86                                Color::BLACK
87                            };
88                            commands.spawn((
89                                Node {
90                                    grid_row: GridPlacement::start(y + 1),
91                                    grid_column: GridPlacement::start(x + 1),
92                                    ..default()
93                                },
94                                Pickable {
95                                    should_block_lower: false,
96                                    is_hoverable: true,
97                                },
98                                TileColor(tile_color),
99                                BackgroundColor(tile_color),
100                            ))
101                            .observe(|over: On<Pointer<Over>>, mut query: Query<&mut BackgroundColor>,| {
102                                if let Ok(mut background_color) = query.get_mut(over.entity) {
103                                    background_color.0 = RED.into();
104                                }
105                            })
106                            .observe(|out: On<Pointer<Out>>, mut query: Query<(&mut BackgroundColor, &TileColor)>| {
107                                if let Ok((mut background_color, tile_color)) = query.get_mut(out.entity) {
108                                    background_color.0 = tile_color.0;
109                                }
110                            });
111                        }
112                    }
113                });
114        });
115}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#240)

#### pub fn [clamp](#method.clamp)(self, min: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), max: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Component-wise clamping of values, similar to [`f32::clamp`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.clamp "method f32::clamp").

Each element in `min` must be less-or-equal to the corresponding element in `max`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

##### Panics

Will panic if `min` is greater than `max` when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/ui/ui\_transform.rs ([line 59](../../../src/ui_transform/ui_transform.rs.html#59))

```rust
34fn button_system(
35    mut interaction_query: Query<
36        (
37            &Interaction,
38            &mut BackgroundColor,
39            Option<&RotateButton>,
40            Option<&ScaleButton>,
41        ),
42        (Changed<Interaction>, With<Button>),
43    >,
44    mut rotator_query: Query<&mut UiTransform, With<TargetNode>>,
45) {
46    for (interaction, mut color, maybe_rotate, maybe_scale) in &mut interaction_query {
47        match *interaction {
48            Interaction::Pressed => {
49                *color = PRESSED_BUTTON.into();
50                if let Some(step) = maybe_rotate {
51                    for mut transform in rotator_query.iter_mut() {
52                        transform.rotation *= step.0;
53                    }
54                }
55                if let Some(step) = maybe_scale {
56                    for mut transform in rotator_query.iter_mut() {
57                        transform.scale += step.0;
58                        transform.scale =
59                            transform.scale.clamp(Vec2::splat(0.25), Vec2::splat(3.0));
60                    }
61                }
62            }
63            Interaction::Hovered => {
64                *color = HOVERED_BUTTON.into();
65            }
66            Interaction::None => {
67                *color = NORMAL_BUTTON.into();
68            }
69        }
70    }
71}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#253)

#### pub fn [min\_element](#method.min_element)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the horizontal minimum of `self`.

In other words this computes `min(x, y, ..)`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#266)

#### pub fn [max\_element](#method.max_element)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the horizontal maximum of `self`.

In other words this computes `max(x, y, ..)`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#275)

#### pub fn [min\_position](#method.min_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first minimum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#287)

#### pub fn [max\_position](#method.max_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first maximum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#300)

#### pub fn [element\_sum](#method.element_sum)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the sum of all elements of `self`.

In other words, this computes `self.x + self.y + ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#309)

#### pub fn [element\_product](#method.element_product)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the product of all elements of `self`.

In other words, this computes `self.x * self.y * ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#320)

#### pub fn [cmpeq](#method.cmpeq)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `==` comparison for each element of `self` and `rhs`.

In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#331)

#### pub fn [cmpne](#method.cmpne)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `!=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#342)

#### pub fn [cmpge](#method.cmpge)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `>=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#353)

#### pub fn [cmpgt](#method.cmpgt)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `>` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#364)

#### pub fn [cmple](#method.cmple)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `<=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#375)

#### pub fn [cmplt](#method.cmplt)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Returns a vector mask containing the result of a `<` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#382)

#### pub fn [abs](#method.abs)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the absolute value of each element of `self`.

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/math/custom\_primitives.rs ([line 441](../../../src/custom_primitives/custom_primitives.rs.html#441))

```rust
435    fn aabb_2d(&self, isometry: impl Into<Isometry2d>) -> Aabb2d {
436        let isometry = isometry.into();
437
438        // The center of the circle at the center of the right wing of the heart
439        let circle_center = isometry.rotation * Vec2::new(self.radius, 0.0);
440        // The maximum X and Y positions of the two circles of the wings of the heart.
441        let max_circle = circle_center.abs() + Vec2::splat(self.radius);
442        // Since the two circles of the heart are mirrored around the origin, the minimum position is the negative of the maximum.
443        let min_circle = -max_circle;
444
445        // The position of the tip at the bottom of the heart
446        let tip_position = isometry.rotation * Vec2::new(0.0, -self.radius * (1. + SQRT_2));
447
448        Aabb2d {
449            min: isometry.translation + min_circle.min(tip_position),
450            max: isometry.translation + max_circle.max(tip_position),
451        }
452    }
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#396)

#### pub fn [signum](#method.signum)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector with elements representing the sign of `self`.

*   `1.0` if the number is positive, `+0.0` or `INFINITY`
*   `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
*   `NAN` if the number is `NAN`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#406)

#### pub fn [copysign](#method.copysign)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector with signs of `rhs` and the magnitudes of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#422)

#### pub fn [is\_negative\_bitmask](#method.is_negative_bitmask)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.

A negative element results in a `1` bit and a positive element in a `0` bit. Element `x` goes into the first lowest bit, element `y` into the second, etc.

An element is negative if it has a negative sign, including -0.0, NaNs with negative sign bit and negative infinity.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#430)

#### pub fn [is\_finite](#method.is_finite)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if, and only if, all elements are finite. If any element is either `NaN`, positive or negative infinity, this will return `false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#439)

#### pub fn [is\_finite\_mask](#method.is_finite_mask)(self) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Performs `is_finite` on each element of self, returning a vector mask of the results.

In other words, this computes `[x.is_finite(), y.is_finite(), ...]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#446)

#### pub fn [is\_nan](#method.is_nan)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if any elements are `NaN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#455)

#### pub fn [is\_nan\_mask](#method.is_nan_mask)(self) -> [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

Performs `is_nan` on each element of self, returning a vector mask of the results.

In other words, this computes `[x.is_nan(), y.is_nan(), ...]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#463)

#### pub fn [length](#method.length)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the length of `self`.

##### [Examples found in repository](#scraped-examples-8)[?](../../../scrape-examples-help.html)

examples/showcase/desk\_toy.rs ([line 260](../../../src/desk_toy/desk_toy.rs.html#260))

```rust
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

Hide additional examples

examples/3d/light\_probe\_blending.rs ([line 523](../../../src/light_probe_blending/light_probe_blending.rs.html#523))

```rust
497fn handle_camera_mode_change(
498    mut commands: Commands,
499    cameras_query: Query<(Entity, &Transform), With<Camera3d>>,
500    sphere_query: Query<&Transform, (With<ReflectiveSphere>, Without<Camera3d>)>,
501    mut help_text_query: Query<&mut Text, With<HelpText>>,
502    mut windows_query: Query<&mut CursorOptions>,
503    mut app_status: ResMut<AppStatus>,
504    mut messages: MessageReader<WidgetClickEvent<CameraMode>>,
505) {
506    let Some(sphere_transform) = sphere_query.iter().next() else {
507        return;
508    };
509
510    let mut any_changes = false;
511    for message in messages.read() {
512        app_status.camera_mode = **message;
513
514        match **message {
515            CameraMode::Orbit => {
516                for (camera_entity, camera_transform) in &cameras_query {
517                    // Convert from Cartesian coordinates back to spherical
518                    // coordinates.
519                    let relative_camera_position =
520                        camera_transform.translation - sphere_transform.translation;
521                    let radius = relative_camera_position.length();
522                    let inclination = atan2(
523                        relative_camera_position.xz().length() / radius,
524                        relative_camera_position.y / radius,
525                    );
526                    let azimuth = atan2(
527                        relative_camera_position.z * relative_camera_position.xz().length_recip(),
528                        relative_camera_position.x * relative_camera_position.xz().length_recip(),
529                    );
530
531                    commands
532                        .entity(camera_entity)
533                        .remove::<FreeCamera>()
534                        .insert(OrbitCamera {
535                            radius,
536                            inclination,
537                            azimuth,
538                        });
539                }
540            }
541
542            CameraMode::Free => {
543                for (camera_entity, _) in &cameras_query {
544                    commands
545                        .entity(camera_entity)
546                        .remove::<OrbitCamera>()
547                        .insert(FreeCamera::default());
548                }
549            }
550        }
551
552        any_changes = true;
553    }
554
555    if any_changes {
556        set_help_text(&app_status, &mut help_text_query);
557
558        // Reset the cursor grab mode, because the free camera controller may
559        // have enabled it, and we don't want the cursor to disappear.
560        for mut cursor_options in &mut windows_query {
561            cursor_options.grab_mode = CursorGrabMode::None;
562            cursor_options.visible = true;
563        }
564    }
565}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#473)

#### pub fn [length\_squared](#method.length_squared)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the squared length of `self`.

This is faster than `length()` as it avoids a square root operation.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#482)

#### pub fn [length\_recip](#method.length_recip)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes `1.0 / length()`.

For valid results, `self` must _not_ be of length zero.

##### [Examples found in repository](#scraped-examples-9)[?](../../../scrape-examples-help.html)

examples/3d/clustered\_decals.rs ([line 407](../../../src/clustered_decals/clustered_decals.rs.html#407))

```rust
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

Hide additional examples

examples/3d/light\_textures.rs ([line 489](../../../src/light_textures/light_textures.rs.html#489))

```rust
461fn process_move_input(
462    mut selections: Query<(&mut Transform, &Selection)>,
463    mouse_buttons: Res<ButtonInput<MouseButton>>,
464    mouse_motion: Res<AccumulatedMouseMotion>,
465    app_status: Res<AppStatus>,
466) {
467    // Only process drags when movement is selected.
468    if !mouse_buttons.pressed(MouseButton::Left) || app_status.drag_mode != DragMode::Move {
469        return;
470    }
471
472    for (mut transform, selection) in &mut selections {
473        if app_status.selection != *selection {
474            continue;
475        }
476
477        // use simple movement for the point light
478        if *selection == Selection::PointLight {
479            transform.translation +=
480                (mouse_motion.delta * Vec2::new(1.0, -1.0) * MOVE_SPEED).extend(0.0);
481            return;
482        }
483
484        let position = transform.translation;
485
486        // Convert to spherical coordinates.
487        let radius = position.length();
488        let mut theta = acos(position.y / radius);
489        let mut phi = position.z.signum() * acos(position.x * position.xz().length_recip());
490
491        // Camera movement is the inverse of object movement.
492        let (phi_factor, theta_factor) = match *selection {
493            Selection::Camera => (1.0, -1.0),
494            _ => (-1.0, 1.0),
495        };
496
497        // Adjust the spherical coordinates. Clamp the inclination to (0, π).
498        phi += phi_factor * mouse_motion.delta.x * MOVE_SPEED;
499        theta = f32::clamp(
500            theta + theta_factor * mouse_motion.delta.y * MOVE_SPEED,
501            0.001,
502            PI - 0.001,
503        );
504
505        // Convert spherical coordinates back to Cartesian coordinates.
506        transform.translation =
507            radius * vec3(sin(theta) * cos(phi), cos(theta), sin(theta) * sin(phi));
508
509        // Look at the center, but preserve the previous roll angle.
510        let roll = transform.rotation.to_euler(EulerRot::YXZ).2;
511        transform.look_at(Vec3::ZERO, Vec3::Y);
512        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
513        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
514    }
515}
```

examples/3d/light\_probe\_blending.rs ([line 527](../../../src/light_probe_blending/light_probe_blending.rs.html#527))

```rust
497fn handle_camera_mode_change(
498    mut commands: Commands,
499    cameras_query: Query<(Entity, &Transform), With<Camera3d>>,
500    sphere_query: Query<&Transform, (With<ReflectiveSphere>, Without<Camera3d>)>,
501    mut help_text_query: Query<&mut Text, With<HelpText>>,
502    mut windows_query: Query<&mut CursorOptions>,
503    mut app_status: ResMut<AppStatus>,
504    mut messages: MessageReader<WidgetClickEvent<CameraMode>>,
505) {
506    let Some(sphere_transform) = sphere_query.iter().next() else {
507        return;
508    };
509
510    let mut any_changes = false;
511    for message in messages.read() {
512        app_status.camera_mode = **message;
513
514        match **message {
515            CameraMode::Orbit => {
516                for (camera_entity, camera_transform) in &cameras_query {
517                    // Convert from Cartesian coordinates back to spherical
518                    // coordinates.
519                    let relative_camera_position =
520                        camera_transform.translation - sphere_transform.translation;
521                    let radius = relative_camera_position.length();
522                    let inclination = atan2(
523                        relative_camera_position.xz().length() / radius,
524                        relative_camera_position.y / radius,
525                    );
526                    let azimuth = atan2(
527                        relative_camera_position.z * relative_camera_position.xz().length_recip(),
528                        relative_camera_position.x * relative_camera_position.xz().length_recip(),
529                    );
530
531                    commands
532                        .entity(camera_entity)
533                        .remove::<FreeCamera>()
534                        .insert(OrbitCamera {
535                            radius,
536                            inclination,
537                            azimuth,
538                        });
539                }
540            }
541
542            CameraMode::Free => {
543                for (camera_entity, _) in &cameras_query {
544                    commands
545                        .entity(camera_entity)
546                        .remove::<OrbitCamera>()
547                        .insert(FreeCamera::default());
548                }
549            }
550        }
551
552        any_changes = true;
553    }
554
555    if any_changes {
556        set_help_text(&app_status, &mut help_text_query);
557
558        // Reset the cursor grab mode, because the free camera controller may
559        // have enabled it, and we don't want the cursor to disappear.
560        for mut cursor_options in &mut windows_query {
561            cursor_options.grab_mode = CursorGrabMode::None;
562            cursor_options.visible = true;
563        }
564    }
565}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#489)

#### pub fn [distance](#method.distance)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the Euclidean distance between two points in space.

##### [Examples found in repository](#scraped-examples-10)[?](../../../scrape-examples-help.html)

examples/showcase/desk\_toy.rs ([line 241](../../../src/desk_toy/desk_toy.rs.html#241))

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
```

Hide additional examples

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

examples/ecs/observers.rs ([line 29](../../../src/observers/observers.rs.html#29))

```rust
8fn main() {
9    App::new()
10        .add_plugins(DefaultPlugins)
11        .init_resource::<SpatialIndex>()
12        .init_resource::<ExplosionsEnabled>()
13        .add_systems(Startup, setup)
14        .add_systems(Update, (draw_shapes, handle_click, toggle_explosions))
15        // Observers are systems that run when an event is "triggered". This observer runs whenever
16        // `ExplodeMines` is triggered.
17        //
18        // Observers can have run conditions, just like systems! This observer only runs when
19        // explosions are enabled. Press Space to toggle.
20        .add_observer(
21            (|explode_mines: On<ExplodeMines>,
22              mines: Query<&Mine>,
23              index: Res<SpatialIndex>,
24              mut commands: Commands| {
25                // Access resources
26                for entity in index.get_nearby(explode_mines.pos) {
27                    // Run queries
28                    let mine = mines.get(entity).unwrap();
29                    if mine.pos.distance(explode_mines.pos) < mine.size + explode_mines.radius {
30                        // And queue commands, including triggering additional events
31                        // Here we trigger the `Explode` event for entity `e`
32                        commands.trigger(Explode { entity });
33                    }
34                }
35            })
36            .run_if(|enabled: Res<ExplosionsEnabled>| enabled.0),
37        )
38        // This observer runs whenever the `Mine` component is added to an entity, and places it in a simple spatial index.
39        .add_observer(on_add_mine)
40        // This observer runs whenever the `Mine` component is removed from an entity (including despawning it)
41        // and removes it from the spatial index.
42        .add_observer(on_remove_mine)
43        .run();
44}
```

examples/2d/cpu\_draw.rs ([line 63](../../../src/cpu_draw/cpu_draw.rs.html#63))

```rust
38fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
39    commands.spawn(Camera2d);
40
41    // Create an image that we are going to draw into
42    let mut image = Image::new_fill(
43        // 2D image of size 256x256
44        Extent3d {
45            width: IMAGE_WIDTH,
46            height: IMAGE_HEIGHT,
47            depth_or_array_layers: 1,
48        },
49        TextureDimension::D2,
50        // Initialize it with a beige color
51        &(css::BEIGE.to_u8_array()),
52        // Use the same encoding as the color we set
53        TextureFormat::Rgba8UnormSrgb,
54        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
55    );
56
57    // To make it extra fancy, we can set the Alpha of each pixel,
58    // so that it fades out in a circular fashion.
59    for y in 0..IMAGE_HEIGHT {
60        for x in 0..IMAGE_WIDTH {
61            let center = Vec2::new(IMAGE_WIDTH as f32 / 2.0, IMAGE_HEIGHT as f32 / 2.0);
62            let max_radius = IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32 / 2.0;
63            let r = Vec2::new(x as f32, y as f32).distance(center);
64            let a = 1.0 - (r / max_radius).clamp(0.0, 1.0);
65
66            // Here we will set the A value by accessing the raw data bytes.
67            // (it is the 4th byte of each pixel, as per our `TextureFormat`)
68
69            // Find our pixel by its coordinates
70            let pixel_bytes = image.pixel_bytes_mut(UVec3::new(x, y, 0)).unwrap();
71            // Convert our f32 to u8
72            pixel_bytes[3] = (a * u8::MAX as f32) as u8;
73        }
74    }
75
76    // Add it to Bevy's assets, so it can be used for rendering
77    // this will give us a handle we can use
78    // (to display it in a sprite, or as part of UI, etc.)
79    let handle = images.add(image);
80
81    // Create a sprite entity using our image
82    commands.spawn(Sprite::from_image(handle.clone()));
83    commands.insert_resource(MyProcGenImage(handle));
84
85    // We're seeding the PRNG here to make this example deterministic for testing purposes.
86    // This isn't strictly required in practical use unless you need your app to be deterministic.
87    let seeded_rng = ChaCha8Rng::seed_from_u64(19878367467712);
88    commands.insert_resource(SeededRng(seeded_rng));
89}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#496)

#### pub fn [distance\_squared](#method.distance_squared)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Compute the squared euclidean distance between two points in space.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#503)

#### pub fn [div\_euclid](#method.div_euclid)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns the element-wise quotient of \[Euclidean division\] of `self` by `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#515)

#### pub fn [rem\_euclid](#method.rem_euclid)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns the element-wise remainder of [Euclidean division](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.rem_euclid "method f32::rem_euclid") of `self` by `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#533)

#### pub fn [normalize](#method.normalize)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns `self` normalized to length 1.0.

For valid results, `self` must be finite and _not_ of length zero, nor very close to zero.

See also [`Self::try_normalize()`](../../prelude/struct.Vec2.html#method.try_normalize "method bevy::prelude::Vec2::try_normalize") and [`Self::normalize_or_zero()`](../../prelude/struct.Vec2.html#method.normalize_or_zero "method bevy::prelude::Vec2::normalize_or_zero").

##### Panics

Will panic if the resulting normalized vector is not finite when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-11)[?](../../../scrape-examples-help.html)

examples/2d/rotation.rs ([line 163](../../../src/rotation/rotation.rs.html#163))

```rust
154fn snap_to_player_system(
155    mut query: Query<&mut Transform, (With<SnapToPlayer>, Without<Player>)>,
156    player_transform: Single<&Transform, With<Player>>,
157) {
158    // Get the player translation in 2D
159    let player_translation = player_transform.translation.xy();
160
161    for mut enemy_transform in &mut query {
162        // Get the vector from the enemy ship to the player ship in 2D and normalize it.
163        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
164
165        // Get the quaternion to rotate from the initial enemy facing direction to the direction
166        // facing the player
167        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));
168
169        // Rotate the enemy to face the player
170        enemy_transform.rotation = rotate_to_player;
171    }
172}
173
174/// Demonstrates rotating an enemy ship to face the player ship at a given rotation speed.
175///
176/// This method uses the vector dot product to determine if the enemy is facing the player and
177/// if not, which way to rotate to face the player. The dot product on two unit length vectors
178/// will return a value between -1.0 and +1.0 which tells us the following about the two vectors:
179///
180/// * If the result is 1.0 the vectors are pointing in the same direction, the angle between them is
181///   0 degrees.
182/// * If the result is 0.0 the vectors are perpendicular, the angle between them is 90 degrees.
183/// * If the result is -1.0 the vectors are parallel but pointing in opposite directions, the angle
184///   between them is 180 degrees.
185/// * If the result is positive the vectors are pointing in roughly the same direction, the angle
186///   between them is greater than 0 and less than 90 degrees.
187/// * If the result is negative the vectors are pointing in roughly opposite directions, the angle
188///   between them is greater than 90 and less than 180 degrees.
189///
190/// It is possible to get the angle by taking the arc cosine (`acos`) of the dot product. It is
191/// often unnecessary to do this though. Beware than `acos` will return `NaN` if the input is less
192/// than -1.0 or greater than 1.0. This can happen even when working with unit vectors due to
193/// floating point precision loss, so it pays to clamp your dot product value before calling
194/// `acos`.
195fn rotate_to_player_system(
196    time: Res<Time>,
197    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
198    player_transform: Single<&Transform, With<Player>>,
199) {
200    // Get the player translation in 2D
201    let player_translation = player_transform.translation.xy();
202
203    for (config, mut enemy_transform) in &mut query {
204        // Get the enemy ship forward vector in 2D (already unit length)
205        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();
206
207        // Get the vector from the enemy ship to the player ship in 2D and normalize it.
208        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
209
210        // Get the dot product between the enemy forward vector and the direction to the player.
211        let forward_dot_player = enemy_forward.dot(to_player);
212
213        // If the dot product is approximately 1.0 then the enemy is already facing the player and
214        // we can early out.
215        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
216            continue;
217        }
218
219        // Get the right vector of the enemy ship in 2D (already unit length)
220        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();
221
222        // Get the dot product of the enemy right vector and the direction to the player ship.
223        // If the dot product is negative them we need to rotate counter clockwise, if it is
224        // positive we need to rotate clockwise. Note that `copysign` will still return 1.0 if the
225        // dot product is 0.0 (because the player is directly behind the enemy, so perpendicular
226        // with the right vector).
227        let right_dot_player = enemy_right.dot(to_player);
228
229        // Determine the sign of rotation from the right dot player. We need to negate the sign
230        // here as the 2D bevy co-ordinate system rotates around +Z, which is pointing out of the
231        // screen. Due to the right hand rule, positive rotation around +Z is counter clockwise and
232        // negative is clockwise.
233        let rotation_sign = -f32::copysign(1.0, right_dot_player);
234
235        // Limit rotation so we don't overshoot the target. We need to convert our dot product to
236        // an angle here so we can get an angle of rotation to clamp against.
237        let max_angle = ops::acos(forward_dot_player.clamp(-1.0, 1.0)); // Clamp acos for safety
238
239        // Calculate angle of rotation with limit
240        let rotation_angle =
241            rotation_sign * (config.rotation_speed * time.delta_secs()).min(max_angle);
242
243        // Rotate the enemy to face the player
244        enemy_transform.rotate_z(rotation_angle);
245    }
246}
```

Hide additional examples

examples/math/custom\_primitives.rs ([line 577](../../../src/custom_primitives/custom_primitives.rs.html#577))

```rust
570    fn perimeter(&self) -> Vec<PerimeterSegment> {
571        let resolution = self.resolution as u32;
572        vec![
573            // The left wing of the heart
574            PerimeterSegment::Smooth {
575                // The normals of the first and last vertices of smooth segments have to be specified manually.
576                first_normal: Vec2::X,
577                last_normal: Vec2::new(-1.0, -1.0).normalize(),
578                // These indices are used to index into the `ATTRIBUTE_POSITION` vec of your 2D mesh.
579                indices: (0..resolution).collect(),
580            },
581            // The bottom tip of the heart
582            PerimeterSegment::Flat {
583                indices: vec![resolution - 1, resolution, resolution + 1],
584            },
585            // The right wing of the heart
586            PerimeterSegment::Smooth {
587                first_normal: Vec2::new(1.0, -1.0).normalize(),
588                last_normal: Vec2::NEG_X,
589                indices: (resolution + 1..2 * resolution).chain([0]).collect(),
590            },
591        ]
592    }
```

examples/showcase/desk\_toy.rs ([line 375](../../../src/desk_toy/desk_toy.rs.html#375))

```rust
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

examples/showcase/breakout.rs ([line 206](../../../src/breakout/breakout.rs.html#206))

```rust
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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#548)

#### pub fn [try\_normalize](#method.try_normalize)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>

Returns `self` normalized to length 1.0 if possible, else returns `None`.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be `None`.

See also [`Self::normalize_or_zero()`](../../prelude/struct.Vec2.html#method.normalize_or_zero "method bevy::prelude::Vec2::normalize_or_zero").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#566)

#### pub fn [normalize\_or](#method.normalize_or)(self, fallback: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns `self` normalized to length 1.0 if possible, else returns a fallback value.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be the fallback value.

See also [`Self::try_normalize()`](../../prelude/struct.Vec2.html#method.try_normalize "method bevy::prelude::Vec2::try_normalize").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#583)

#### pub fn [normalize\_or\_zero](#method.normalize_or_zero)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns `self` normalized to length 1.0 if possible, else returns zero.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be zero.

See also [`Self::try_normalize()`](../../prelude/struct.Vec2.html#method.try_normalize "method bevy::prelude::Vec2::try_normalize").

##### [Examples found in repository](#scraped-examples-12)[?](../../../scrape-examples-help.html)

examples/camera/2d\_top\_down\_camera.rs ([line 114](../../../src/2d_top_down_camera/2d_top_down_camera.rs.html#114))

```rust
88fn move_player(
89    mut player: Single<&mut Transform, With<Player>>,
90    time: Res<Time>,
91    kb_input: Res<ButtonInput<KeyCode>>,
92) {
93    let mut direction = Vec2::ZERO;
94
95    if kb_input.pressed(KeyCode::KeyW) {
96        direction.y += 1.;
97    }
98
99    if kb_input.pressed(KeyCode::KeyS) {
100        direction.y -= 1.;
101    }
102
103    if kb_input.pressed(KeyCode::KeyA) {
104        direction.x -= 1.;
105    }
106
107    if kb_input.pressed(KeyCode::KeyD) {
108        direction.x += 1.;
109    }
110
111    // Progressively update the player's position over time. Normalize the
112    // direction vector to prevent it from exceeding a magnitude of 1 when
113    // moving diagonally.
114    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
115    player.translation += move_delta.extend(0.);
116}
```

Hide additional examples

examples/3d/tonemapping.rs ([line 256](../../../src/tonemapping/tonemapping.rs.html#256))

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#592)

#### pub fn [normalize\_and\_length](#method.normalize_and_length)(self) -> ([Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Returns `self` normalized to length 1.0 and the length of `self`.

If `self` is zero length then `(Self::X, 0.0)` is returned.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#607)

#### pub fn [is\_normalized](#method.is_normalized)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns whether `self` is length `1.0` or not.

Uses a precision threshold of approximately `1e-4`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#620)

#### pub fn [project\_onto](#method.project_onto)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns the vector projection of `self` onto `rhs`.

`rhs` must be of non-zero length.

##### Panics

Will panic if `rhs` is zero length when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#639)

#### pub fn [reject\_from](#method.reject_from)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns the vector rejection of `self` from `rhs`.

The vector rejection is the vector perpendicular to the projection of `self` onto `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.

`rhs` must be of non-zero length.

##### Panics

Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#652)

#### pub fn [project\_onto\_normalized](#method.project_onto_normalized)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns the vector projection of `self` onto `rhs`.

`rhs` must be normalized.

##### Panics

Will panic if `rhs` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#670)

#### pub fn [reject\_from\_normalized](#method.reject_from_normalized)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns the vector rejection of `self` from `rhs`.

The vector rejection is the vector perpendicular to the projection of `self` onto `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.

`rhs` must be normalized.

##### Panics

Will panic if `rhs` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#678)

#### pub fn [round](#method.round)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the nearest integer to a number for each element of `self`. Round half-way cases away from 0.0.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#689)

#### pub fn [floor](#method.floor)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the largest integer less than or equal to a number for each element of `self`.

##### [Examples found in repository](#scraped-examples-13)[?](../../../scrape-examples-help.html)

examples/asset/asset\_saving.rs ([line 236](../../../src/asset_saving/asset_saving.rs.html#236))

```rust
208fn try_plot(
209    event: On<TryPlot>,
210    sprite: Query<(&Sprite, &Anchor, &GlobalTransform), With<SpriteToSave>>,
211    camera: Single<(&Camera, &GlobalTransform)>,
212    texture_atlases: Res<Assets<TextureAtlasLayout>>,
213    draw_color: Res<DrawColor>,
214    mut images: ResMut<Assets<Image>>,
215) {
216    let Ok((sprite, anchor, sprite_transform)) = sprite.get(event.entity) else {
217        return;
218    };
219    let (camera, camera_transform) = camera.into_inner();
220    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, event.location.position)
221    else {
222        return;
223    };
224    let relative_to_sprite = sprite_transform
225        .affine()
226        .inverse()
227        .transform_point3(world_position.extend(0.0));
228    let Ok(pixel_space) = sprite.compute_pixel_space_point(
229        relative_to_sprite.xy(),
230        *anchor,
231        &images,
232        &texture_atlases,
233    ) else {
234        return;
235    };
236    let pixel_coordinates = pixel_space.floor().as_uvec2();
237    let mut image = images.get_mut(&sprite.image).unwrap();
238    // For an actual drawing app, you'd at least draw a line from the last point, but this is
239    // simpler.
240    image
241        .set_color_at(pixel_coordinates.x, pixel_coordinates.y, draw_color.0)
242        .unwrap();
243}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#700)

#### pub fn [ceil](#method.ceil)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the smallest integer greater than or equal to a number for each element of `self`.

##### [Examples found in repository](#scraped-examples-14)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_cubes.rs ([line 259](../../../src/many_cubes/many_cubes.rs.html#259))

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#711)

#### pub fn [trunc](#method.trunc)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the integer part each element of `self`. This means numbers are always truncated towards zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#723)

#### pub fn [step](#method.step)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing `0.0` if `rhs < self` and 1.0 otherwise.

Similar to glsl’s step(edge, x), which translates into edge.step(x)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#730)

#### pub fn [saturate](#method.saturate)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing all elements of `self` clamped to the range of `[0, 1]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#742)

#### pub fn [fract](#method.fract)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the fractional part of the vector as `self - self.trunc()`.

Note that this differs from the GLSL implementation of `fract` which returns `self - self.floor()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#754)

#### pub fn [fract\_gl](#method.fract_gl)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the fractional part of the vector as `self - self.floor()`.

Note that this differs from the Rust implementation of `fract` which returns `self - self.trunc()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#762)

#### pub fn [exp](#method.exp)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing `e^self` (the exponential function) for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#769)

#### pub fn [exp2](#method.exp2)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing `2^self` for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#777)

#### pub fn [ln](#method.ln)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the natural logarithm for each element of `self`. This returns NaN when the element is negative and negative infinity when the element is zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#785)

#### pub fn [log2](#method.log2)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the base 2 logarithm for each element of `self`. This returns NaN when the element is negative and negative infinity when the element is zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#792)

#### pub fn [powf](#method.powf)(self, n: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing each element of `self` raised to the power of `n`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#800)

#### pub fn [sqrt](#method.sqrt)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the square root for each element of `self`. This returns NaN when the element is negative.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#807)

#### pub fn [cos](#method.cos)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the cosine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#814)

#### pub fn [sin](#method.sin)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the sine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#821)

#### pub fn [sin\_cos](#method.sin_cos)(self) -> ([Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Returns a tuple of two vectors containing the sine and cosine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#831)

#### pub fn [recip](#method.recip)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#846)

#### pub fn [lerp](#method.lerp)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), s: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs a linear interpolation between `self` and `rhs` based on the value `s`.

When `s` is `0.0`, the result will be equal to `self`. When `s` is `1.0`, the result will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly extrapolated.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#856)

#### pub fn [move\_towards](#method.move_towards)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), d: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Moves towards `rhs` based on the value `d`.

When `d` is `0.0`, the result will be equal to `self`. When `d` is equal to `self.distance(rhs)`, the result will be equal to `rhs`. Will not go past `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#871)

#### pub fn [midpoint](#method.midpoint)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Calculates the midpoint between `self` and `rhs`.

The midpoint is the average of, or halfway point between, two vectors. `a.midpoint(b)` should yield the same result as `a.lerp(b, 0.5)` while being slightly cheaper to compute.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#886)

#### pub fn [abs\_diff\_eq](#method.abs_diff_eq)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), max\_abs\_diff: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the absolute difference of all elements between `self` and `rhs` is less than or equal to `max_abs_diff`.

This can be used to compare if two vectors contain similar elements. It works best when comparing with a known value. The `max_abs_diff` that should be used used depends on the values being compared against.

For more see [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#897)

#### pub fn [clamp\_length](#method.clamp_length)(self, min: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), max: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector with a length no less than `min` and no more than `max`.

##### Panics

Will panic if `min` is greater than `max`, or if either `min` or `max` is negative, when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#917)

#### pub fn [clamp\_length\_max](#method.clamp_length_max)(self, max: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector with a length no more than `max`.

##### Panics

Will panic if `max` is negative when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#934)

#### pub fn [clamp\_length\_min](#method.clamp_length_min)(self, min: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector with a length no less than `min`.

##### Panics

Will panic if `min` is negative when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#953)

#### pub fn [mul\_add](#method.mul_add)(self, a: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), b: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding error, yielding a more accurate result than an unfused multiply-add.

Using `mul_add` _may_ be more performant than an unfused multiply-add if the target architecture has a dedicated fma CPU instruction. However, this is not always true, and will be heavily dependant on designing algorithms with specific target hardware in mind.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#970)

#### pub fn [reflect](#method.reflect)(self, normal: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns the reflection vector for a given incident vector `self` and surface normal `normal`.

`normal` must be normalized.

##### Panics

Will panic if `normal` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#986)

#### pub fn [refract](#method.refract)(self, normal: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), eta: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns the refraction direction for a given incident vector `self`, surface normal `normal` and ratio of indices of refraction, `eta`. When total internal reflection occurs, a zero vector will be returned.

`self` and `normal` must be normalized.

##### Panics

Will panic if `self` or `normal` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1004)

#### pub fn [from\_angle](#method.from_angle)(angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in conjunction with the [`rotate()`](../../prelude/struct.Vec2.html#method.rotate "method bevy::prelude::Vec2::rotate") method, e.g. `Vec2::from_angle(PI).rotate(Vec2::Y)` will create the vector `[-1, 0]` and rotate [`Vec2::Y`](../../prelude/struct.Vec2.html#associatedconstant.Y "associated constant bevy::prelude::Vec2::Y") around it returning `-Vec2::Y`.

##### [Examples found in repository](#scraped-examples-15)[?](../../../scrape-examples-help.html)

examples/3d/reflection\_probes.rs ([line 353](../../../src/reflection_probes/reflection_probes.rs.html#353))

```rust
343fn rotate_camera(
344    time: Res<Time>,
345    mut camera_query: Query<&mut Transform, With<Camera3d>>,
346    app_status: Res<AppStatus>,
347) {
348    if !app_status.rotating {
349        return;
350    }
351
352    for mut transform in camera_query.iter_mut() {
353        transform.translation = Vec2::from_angle(time.delta_secs() * PI / 5.0)
354            .rotate(transform.translation.xz())
355            .extend(transform.translation.y)
356            .xzy();
357        transform.look_at(Vec3::ZERO, Vec3::Y);
358    }
359}
```

Hide additional examples

examples/3d/irradiance\_volumes.rs ([line 359](../../../src/irradiance_volumes/irradiance_volumes.rs.html#359))

```rust
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
```

examples/2d/cpu\_draw.rs ([line 120](../../../src/cpu_draw/cpu_draw.rs.html#120))

```rust
92fn draw(
93    my_handle: Res<MyProcGenImage>,
94    mut images: ResMut<Assets<Image>>,
95    // Used to keep track of where we are
96    mut i: Local<u32>,
97    mut draw_color: Local<Color>,
98    mut seeded_rng: ResMut<SeededRng>,
99) {
100    if *i == 0 {
101        // Generate a random color on first run.
102        *draw_color = Color::linear_rgb(
103            seeded_rng.0.random(),
104            seeded_rng.0.random(),
105            seeded_rng.0.random(),
106        );
107    }
108
109    // Get the image from Bevy's asset storage.
110    let mut image = images.get_mut(&my_handle.0).expect("Image not found");
111
112    // Compute the position of the pixel to draw.
113
114    let center = Vec2::new(IMAGE_WIDTH as f32 / 2.0, IMAGE_HEIGHT as f32 / 2.0);
115    let max_radius = IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32 / 2.0;
116    let rot_speed = 0.0123;
117    let period = 0.12345;
118
119    let r = ops::sin(*i as f32 * period) * max_radius;
120    let xy = Vec2::from_angle(*i as f32 * rot_speed) * r + center;
121    let (x, y) = (xy.x as u32, xy.y as u32);
122
123    // Get the old color of that pixel.
124    let old_color = image.get_color_at(x, y).unwrap();
125
126    // If the old color is our current color, change our drawing color.
127    let tolerance = 1.0 / 255.0;
128    if old_color.distance(&draw_color) <= tolerance {
129        *draw_color = Color::linear_rgb(
130            seeded_rng.0.random(),
131            seeded_rng.0.random(),
132            seeded_rng.0.random(),
133        );
134    }
135
136    // Set the new color, but keep old alpha value from image.
137    image
138        .set_color_at(x, y, draw_color.with_alpha(old_color.alpha()))
139        .unwrap();
140
141    *i += 1;
142}
```

examples/3d/clustered\_decal\_maps.rs ([line 331](../../../src/clustered_decal_maps/clustered_decal_maps.rs.html#331))

```rust
281fn spawn_decal(
282    mut commands: Commands,
283    app_status: Res<AppStatus>,
284    app_textures: Res<AppTextures>,
285    time: Res<Time>,
286    mut decal_spawn_timer: Local<Option<Timer>>,
287    mut seeded_rng: ResMut<SeededRng>,
288) {
289    // Tick the decal spawn timer. Check to see if we should spawn a new decal,
290    // and bail out if it's not yet time to.
291    let decal_spawn_timer = decal_spawn_timer
292        .get_or_insert_with(|| Timer::new(Duration::from_millis(1000), TimerMode::Repeating));
293    decal_spawn_timer.tick(time.delta());
294    if !decal_spawn_timer.just_finished() {
295        return;
296    }
297
298    // Generate a random position along the plane.
299    let decal_position = vec3(
300        seeded_rng.0.random_range(-PLANE_HALF_SIZE..PLANE_HALF_SIZE),
301        seeded_rng.0.random_range(-PLANE_HALF_SIZE..PLANE_HALF_SIZE),
302        0.0,
303    );
304
305    // Generate a random size for the decal.
306    let decal_size = seeded_rng.0.random_range(DECAL_MIN_SIZE..DECAL_MAX_SIZE);
307
308    // Generate a random rotation for the decal.
309    let theta = seeded_rng.0.random_range(0.0f32..PI);
310
311    // Now spawn the decal.
312    commands.spawn((
313        // Apply the textures.
314        ClusteredDecal {
315            base_color_texture: Some(app_textures.decal_base_color_texture.clone()),
316            normal_map_texture: Some(app_textures.decal_normal_map_texture.clone()),
317            metallic_roughness_texture: Some(
318                app_textures.decal_metallic_roughness_map_texture.clone(),
319            ),
320            emissive_texture: if app_status.emissive_decals {
321                Some(app_textures.decal_emissive_texture.clone())
322            } else {
323                None
324            },
325            ..ClusteredDecal::default()
326        },
327        // Spawn the decal at the right place. Note that the scale is initially
328        // zero; we'll animate it later.
329        Transform::from_translation(decal_position)
330            .with_scale(Vec3::ZERO)
331            .looking_to(Vec3::Z, Vec3::ZERO.with_xy(Vec2::from_angle(theta))),
332        // Create the component that tracks the animation state.
333        ExampleDecal {
334            size: decal_size,
335            state: ExampleDecalState::AnimatingIn(Timer::new(
336                DECAL_ANIMATE_IN_DURATION,
337                TimerMode::Once,
338            )),
339        },
340    ));
341}
```

examples/gizmos/2d\_gizmos.rs ([line 111](../../../src/2d_gizmos/2d_gizmos.rs.html#111))

```rust
41fn draw_example_collection(
42    mut gizmos: Gizmos,
43    mut my_gizmos: Gizmos<MyRoundGizmos>,
44    time: Res<Time>,
45) {
46    let sin_t_scaled = ops::sin(time.elapsed_secs()) * 50.;
47    gizmos.line_2d(Vec2::Y * -sin_t_scaled, Vec2::splat(-80.), RED);
48    gizmos.ray_2d(Vec2::Y * sin_t_scaled, Vec2::splat(80.), LIME);
49
50    gizmos
51        .grid_2d(
52            Isometry2d::IDENTITY,
53            UVec2::new(16, 9),
54            Vec2::new(80., 80.),
55            // Dark gray
56            LinearRgba::gray(0.05),
57        )
58        .outer_edges();
59
60    // Triangle
61    gizmos.linestrip_gradient_2d([
62        (Vec2::Y * 300., BLUE),
63        (Vec2::new(-255., -155.), RED),
64        (Vec2::new(255., -155.), LIME),
65        (Vec2::Y * 300., BLUE),
66    ]);
67
68    gizmos.rect_2d(Isometry2d::IDENTITY, Vec2::splat(650.), BLACK);
69
70    gizmos.cross_2d(Vec2::new(-160., 120.), 12., FUCHSIA);
71
72    let domain = Interval::EVERYWHERE;
73    let curve = FunctionCurve::new(domain, |t| Vec2::new(t, ops::sin(t / 25.0) * 100.0));
74    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 50.0) as usize;
75    let times_and_colors = (0..=resolution)
76        .map(|n| n as f32 / resolution as f32)
77        .map(|t| (t - 0.5) * 600.0)
78        .map(|t| (t, TEAL.mix(&HOT_PINK, (t + 300.0) / 600.0)));
79    gizmos.curve_gradient_2d(curve, times_and_colors);
80
81    my_gizmos
82        .rounded_rect_2d(Isometry2d::IDENTITY, Vec2::splat(630.), BLACK)
83        .corner_radius(ops::cos(time.elapsed_secs() / 3.) * 100.);
84
85    // Circles have 32 line-segments by default.
86    // You may want to increase this for larger circles.
87    my_gizmos
88        .circle_2d(Isometry2d::IDENTITY, 300., NAVY)
89        .resolution(64);
90
91    my_gizmos.ellipse_2d(
92        Rot2::radians(time.elapsed_secs() % TAU),
93        Vec2::new(100., 200.),
94        YELLOW_GREEN,
95    );
96
97    // Arcs default resolution is linearly interpolated between
98    // 1 and 32, using the arc length as scalar.
99    my_gizmos.arc_2d(
100        Rot2::radians(sin_t_scaled / 10.),
101        FRAC_PI_2,
102        310.,
103        ORANGE_RED,
104    );
105    my_gizmos.arc_2d(Isometry2d::IDENTITY, FRAC_PI_2, 80.0, ORANGE_RED);
106    my_gizmos.long_arc_2d_between(Vec2::ZERO, Vec2::X * 20.0, Vec2::Y * 20.0, ORANGE_RED);
107    my_gizmos.short_arc_2d_between(Vec2::ZERO, Vec2::X * 40.0, Vec2::Y * 40.0, ORANGE_RED);
108
109    gizmos.arrow_2d(
110        Vec2::ZERO,
111        Vec2::from_angle(sin_t_scaled / -10. + PI / 2.) * 50.,
112        YELLOW,
113    );
114
115    // You can create more complex arrows using the arrow builder.
116    gizmos
117        .arrow_2d(
118            Vec2::ZERO,
119            Vec2::from_angle(sin_t_scaled / -10.) * 50.,
120            GREEN,
121        )
122        .with_double_end()
123        .with_tip_length(10.);
124}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1014)

#### pub fn [to\_angle](#method.to_angle)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the angle (in radians) of this vector in the range `[-π, +π]`.

The input does not need to be a unit vector however it must be non-zero.

##### [Examples found in repository](#scraped-examples-16)[?](../../../scrape-examples-help.html)

examples/2d/rotate\_to\_cursor.rs ([line 56](../../../src/rotate_to_cursor/rotate_to_cursor.rs.html#56))

```rust
38fn player_movement_system(
39    mut player: Single<&mut Transform, With<Player>>,
40    camera_query: Single<(&Camera, &GlobalTransform)>,
41    window: Single<&Window>,
42) {
43    let (camera, camera_transform) = *camera_query;
44
45    if let Some(cursor_position) = window.cursor_position()
46        // Calculate a world position based on the cursor's position.
47        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
48    {
49        // The angle an entity needs to rotate to face a point is defined
50        // by the vector between the two points (Vec2 - Vec2), which we can then
51        // turn into radians using to_angle.
52        //
53        // FRAC_PI_2 is because our sprite's ship is facing "up" so we rotate it an additional 90 degrees
54        // so that it faces the cursor.
55        player.rotation = Quat::from_rotation_z(
56            (cursor_world_pos - player.translation.xy()).to_angle() - FRAC_PI_2,
57        );
58    }
59}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1023)

#### pub fn [angle\_to](#method.angle_to)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the angle of rotation (in radians) from `self` to `rhs` in the range `[-π, +π]`.

The inputs do not need to be unit vectors however they must be non-zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1034)

#### pub fn [perp](#method.perp)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a vector that is equal to `self` rotated by 90 degrees.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1048)

#### pub fn [perp\_dot](#method.perp_dot)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The perpendicular dot product of `self` and `rhs`. Also known as the wedge product, 2D cross product, and determinant.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1061)

#### pub fn [rotate](#method.rotate)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns `rhs` rotated by the angle of `self`. If `self` is normalized, then this just rotation. This is what you usually want. Otherwise, it will be like a rotation with a multiplication by `self`’s length.

This can be used in conjunction with the [`from_angle()`](../../prelude/struct.Vec2.html#method.from_angle "associated function bevy::prelude::Vec2::from_angle") method, e.g. `Vec2::from_angle(PI).rotate(Vec2::Y)` will create the vector `[-1, 0]` and rotate [`Vec2::Y`](../../prelude/struct.Vec2.html#associatedconstant.Y "associated constant bevy::prelude::Vec2::Y") around it returning `-Vec2::Y`.

##### [Examples found in repository](#scraped-examples-17)[?](../../../scrape-examples-help.html)

examples/3d/reflection\_probes.rs ([line 354](../../../src/reflection_probes/reflection_probes.rs.html#354))

```rust
343fn rotate_camera(
344    time: Res<Time>,
345    mut camera_query: Query<&mut Transform, With<Camera3d>>,
346    app_status: Res<AppStatus>,
347) {
348    if !app_status.rotating {
349        return;
350    }
351
352    for mut transform in camera_query.iter_mut() {
353        transform.translation = Vec2::from_angle(time.delta_secs() * PI / 5.0)
354            .rotate(transform.translation.xz())
355            .extend(transform.translation.y)
356            .xzy();
357        transform.look_at(Vec3::ZERO, Vec3::Y);
358    }
359}
```

Hide additional examples

examples/3d/irradiance\_volumes.rs ([line 360](../../../src/irradiance_volumes/irradiance_volumes.rs.html#360))

```rust
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
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1075)

#### pub fn [rotate\_towards](#method.rotate_towards)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), max\_angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Rotates towards `rhs` up to `max_angle` (in radians).

When `max_angle` is `0.0`, the result will be equal to `self`. When `max_angle` is equal to `self.angle_between(rhs)`, the result will be parallel to `rhs`. If `max_angle` is negative, rotates towards the exact opposite of `rhs`. Will not go past the target.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1086)

#### pub fn [as\_dvec2](#method.as_dvec2)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

Casts all elements of `self` to `f64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1093)

#### pub fn [as\_i8vec2](#method.as_i8vec2)(self) -> [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

Casts all elements of `self` to `i8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1100)

#### pub fn [as\_u8vec2](#method.as_u8vec2)(self) -> [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

Casts all elements of `self` to `u8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1107)

#### pub fn [as\_i16vec2](#method.as_i16vec2)(self) -> [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

Casts all elements of `self` to `i16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1114)

#### pub fn [as\_u16vec2](#method.as_u16vec2)(self) -> [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

Casts all elements of `self` to `u16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1121)

#### pub fn [as\_ivec2](#method.as_ivec2)(self) -> [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

Casts all elements of `self` to `i32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1128)

#### pub fn [as\_uvec2](#method.as_uvec2)(self) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Casts all elements of `self` to `u32`.

##### [Examples found in repository](#scraped-examples-18)[?](../../../scrape-examples-help.html)

examples/asset/asset\_saving.rs ([line 236](../../../src/asset_saving/asset_saving.rs.html#236))

```rust
208fn try_plot(
209    event: On<TryPlot>,
210    sprite: Query<(&Sprite, &Anchor, &GlobalTransform), With<SpriteToSave>>,
211    camera: Single<(&Camera, &GlobalTransform)>,
212    texture_atlases: Res<Assets<TextureAtlasLayout>>,
213    draw_color: Res<DrawColor>,
214    mut images: ResMut<Assets<Image>>,
215) {
216    let Ok((sprite, anchor, sprite_transform)) = sprite.get(event.entity) else {
217        return;
218    };
219    let (camera, camera_transform) = camera.into_inner();
220    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, event.location.position)
221    else {
222        return;
223    };
224    let relative_to_sprite = sprite_transform
225        .affine()
226        .inverse()
227        .transform_point3(world_position.extend(0.0));
228    let Ok(pixel_space) = sprite.compute_pixel_space_point(
229        relative_to_sprite.xy(),
230        *anchor,
231        &images,
232        &texture_atlases,
233    ) else {
234        return;
235    };
236    let pixel_coordinates = pixel_space.floor().as_uvec2();
237    let mut image = images.get_mut(&sprite.image).unwrap();
238    // For an actual drawing app, you'd at least draw a line from the last point, but this is
239    // simpler.
240    image
241        .set_color_at(pixel_coordinates.x, pixel_coordinates.y, draw_color.0)
242        .unwrap();
243}
```

Hide additional examples

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

examples/stress\_tests/many\_cubes.rs ([line 260](../../../src/many_cubes/many_cubes.rs.html#260))

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1135)

#### pub fn [as\_i64vec2](#method.as_i64vec2)(self) -> [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

Casts all elements of `self` to `i64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1142)

#### pub fn [as\_u64vec2](#method.as_u64vec2)(self) -> [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

Casts all elements of `self` to `u64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1149)

#### pub fn [as\_isizevec2](#method.as_isizevec2)(self) -> [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

Casts all elements of `self` to `isize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1156)

#### pub fn [as\_usizevec2](#method.as_usizevec2)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Casts all elements of `self` to `usize`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1438)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1439)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1441)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1449)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1450)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1452)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1457)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1458)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1460)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1549)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1550)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1552)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1557)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1558)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1560)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1499)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1500)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1502)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1507)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1508)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1510)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1465)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1466)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1468)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1538)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1539)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1541)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1565)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1566)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1568)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1488)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1489)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1491)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1515)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1516)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1518)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1473)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1475)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1481)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1483)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1531)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1533)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1523)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1525)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#83)

### impl [Animatable](../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#83)

#### fn [interpolate](../../prelude/trait.Animatable.html#tymethod.interpolate)(a: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), b: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Interpolates between `a` and `b` with an interpolation factor of `time`. [Read more](../../prelude/trait.Animatable.html#tymethod.interpolate)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#83)

#### fn [blend](../../prelude/trait.Animatable.html#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](../../prelude/struct.BlendInput.html "struct bevy::prelude::BlendInput")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>>) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Blends one or more values together. [Read more](../../prelude/trait.Animatable.html#tymethod.blend)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1850)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1852)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [AsMutVectorParts](../../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

#### fn [as\_mut\_parts](../../render/render_resource/encase/vector/trait.AsMutVectorParts.html#tymethod.as_mut_parts)(&mut self) -> &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1843)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1845)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [AsRefVectorParts](../../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

#### fn [as\_ref\_parts](../../render/render_resource/encase/vector/trait.AsRefVectorParts.html#tymethod.as_ref_parts)(&self) -> &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#20)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#20)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#20)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [CreateFrom](../../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [FromVectorParts](../../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](../../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

#### fn [create\_from](../../render/render_resource/encase/internal/trait.CreateFrom.html#tymethod.create_from)<B>(reader: &mut [Reader](../../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where B: [BufferRef](../../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1949)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1950)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1161)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1163)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Deserialize expects a sequence of 2 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1939)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1940)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1168)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1169)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1171)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1179)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1180)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1182)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1187)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1188)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1190)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1279)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1280)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1282)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1287)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1288)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1290)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1229)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1230)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1232)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1237)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1238)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1240)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1195)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1196)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1198)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1268)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1269)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1271)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1295)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1296)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1298)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1218)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1219)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1221)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1245)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1246)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1248)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1203)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1205)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1211)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1213)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1261)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1263)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1253)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1255)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1972)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1974)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: ([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1986)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1988)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#305)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Dir2](../../prelude/struct.Dir2.html "struct bevy::prelude::Dir2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#306)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Dir2](../../prelude/struct.Dir2.html "struct bevy::prelude::Dir2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1965)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1967)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec2.rs.html#1986)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec2.rs.html#1988)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#195)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Isometry2d](../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#197)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(translation: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Isometry2d](../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/bounds.rs.html#66)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [TextBounds](../../text/struct.TextBounds.html "struct bevy::text::TextBounds")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/bounds.rs.html#68)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [TextBounds](../../text/struct.TextBounds.html "struct bevy::text::TextBounds")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#425)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [ScrollPosition](../../prelude/struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#426)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [ScrollPosition](../../prelude/struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#288)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Anchor](../../sprite/struct.Anchor.html "struct bevy::sprite::Anchor")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#289)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Anchor](../../sprite/struct.Anchor.html "struct bevy::sprite::Anchor")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1958)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1960)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(a: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### type [This](../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The type to convert into. [Read more](../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [from\_arg](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2") as [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1582)

### impl [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Polyline2d](../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1583)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iter: I) -> [Polyline2d](../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1910)

### impl [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Polygon](../../prelude/struct.Polygon.html "struct bevy::prelude::Polygon")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1911)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iter: I) -> [Polygon](../../prelude/struct.Polygon.html "struct bevy::prelude::Polygon")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [from\_reflect](../../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [FromVectorParts](../../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

#### fn [from\_parts](../../render/render_resource/encase/vector/trait.FromVectorParts.html#tymethod.from_parts)(parts: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [GetOwnership](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [ownership](../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [get\_type\_registration](../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [register\_type\_dependencies](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1916)

### impl [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1917)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The returned type after indexing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1919)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1928)

### impl [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1930)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &mut <[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [IntoReturn](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [into\_return](../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): 'into\_return,

Converts [`Self`](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1303)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1304)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1306)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1314)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1315)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1317)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1322)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1323)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1325)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1414)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1415)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1417)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1422)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1423)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1425)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#659)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Mat2](../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#660)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#662)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#667)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Mat2](../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#668)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#670)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1364)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1365)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1367)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1372)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1373)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1375)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1330)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1331)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1333)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1403)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1404)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1406)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1430)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1431)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1433)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#651)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Mat2](../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#652)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#654)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> <[Mat2](../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#675)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Mat2](../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#676)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat2.rs.html#678)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#221)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Isometry2d](../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#222)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#225)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> <[Isometry2d](../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rotation2d.rs.html#498)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Rot2](../../prelude/struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rotation2d.rs.html#502)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> <[Rot2](../../prelude/struct.Rot2.html "struct bevy::prelude::Rot2") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Rotates a [`Vec2`](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2") by a [`Rot2`](../../prelude/struct.Rot2.html "struct bevy::prelude::Rot2").

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rotation2d.rs.html#499)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#305)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#306)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#309)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, value: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1353)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1354)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1356)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1380)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1381)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1383)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1338)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1340)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1346)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1348)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1396)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1398)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1388)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1390)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1897)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1898)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1900)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1908)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1909)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1911)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#307)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#309)

#### fn [norm](../trait.NormedVectorSpace.html#tymethod.norm)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The size of this element. The return value should always be nonnegative.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#314)

#### fn [norm\_squared](../trait.NormedVectorSpace.html#method.norm_squared)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The squared norm of this element. Computing this is often faster than computing [`NormedVectorSpace::norm`](../trait.NormedVectorSpace.html#tymethod.norm "method bevy::math::NormedVectorSpace::norm").

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#259)

#### fn [distance](../trait.NormedVectorSpace.html#method.distance)(self, rhs: Self) -> Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The distance between this element and another, as determined by the norm.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#266)

#### fn [distance\_squared](../trait.NormedVectorSpace.html#method.distance_squared)(self, rhs: Self) -> Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The squared distance between this element and another, as determined by the norm. Note that this is often faster to compute in practice than [`NormedVectorSpace::distance`](../trait.NormedVectorSpace.html#method.distance "method bevy::math::NormedVectorSpace::distance").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#20)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#20)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [get\_represented\_type\_info](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [try\_apply](../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [reflect\_kind](../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [reflect\_ref](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [reflect\_owned](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>) -> [ReflectOwned](../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [try\_into\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [try\_as\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [try\_as\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [into\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [as\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [as\_partial\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#251)

#### fn [reflect\_partial\_eq](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [reflect\_partial\_cmp](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#251)

#### fn [debug](../../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#251)

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#21)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1877)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1879-1881)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1887)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1889-1891)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [ReadFrom](../../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [AsMutVectorParts](../../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](../../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

#### fn [read\_from](../../render/render_resource/encase/internal/trait.ReadFrom.html#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](../../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](../../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [into\_any](../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [as\_any](../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [as\_any\_mut](../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [into\_reflect](../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [as\_reflect](../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [as\_reflect\_mut](../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [set](../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1708)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1709)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1711)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1719)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1720)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1722)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1727)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1728)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1730)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1819)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1820)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1822)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1827)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1828)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1830)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1769)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1770)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1772)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1777)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1778)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1780)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1735)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1736)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1738)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1808)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1809)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1811)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1835)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1836)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1838)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1758)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1759)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1761)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1785)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1786)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1788)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1743)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1745)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1751)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1753)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1801)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1803)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1793)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1795)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#639)

### impl [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#639)

#### type [Sampler](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html#associatedtype.Sampler) = UniformVec2<[UniformFloat](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/float/struct.UniformFloat.html "struct rand::distr::uniform::float::UniformFloat")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>>

The `UniformSampler` implementation supporting type `X`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Serialize as a sequence of 2 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [ShaderSize](../../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](../../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#232)

#### const [SHADER\_SIZE](../../render/render_resource/trait.ShaderSize.html#associatedconstant.SHADER_SIZE): [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> = \_

Represents [WGSL Size](https://gpuweb.github.io/gpuweb/wgsl/#alignment-and-size) (equivalent to [`ShaderType::min_size`](../../render/render_resource/trait.ShaderType.html#method.min_size "associated function bevy::render::render_resource::ShaderType::min_size"))

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [field](../../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [field\_mut](../../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [field\_at](../../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [field\_at\_mut](../../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [name\_at](../../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [index\_of\_name](../../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [field\_len](../../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [iter\_fields](../../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [to\_dynamic\_struct](../../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#20)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1573)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1574)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1576)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1584)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1585)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1587)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1592)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1593)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1595)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1684)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1685)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1687)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1692)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1693)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1695)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1634)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1635)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1637)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1642)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1643)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1645)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1600)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1601)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1603)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1673)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1674)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1676)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1700)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1701)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1703)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1623)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1624)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1626)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1650)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1651)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1653)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1608)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1610)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1616)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1618)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1666)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1668)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1658)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1660)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1857)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1859-1861)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1867)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#1869-1871)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/aspect_ratio.rs.html#82)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [AspectRatio](../struct.AspectRatio.html "struct bevy::math::AspectRatio")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/aspect_ratio.rs.html#83)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = AspectRatioError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/aspect_ratio.rs.html#86)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[AspectRatio](../struct.AspectRatio.html "struct bevy::math::AspectRatio"), <[AspectRatio](../struct.AspectRatio.html "struct bevy::math::AspectRatio") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#297)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> for [Dir2](../../prelude/struct.Dir2.html "struct bevy::prelude::Dir2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#298)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [InvalidDirectionError](../enum.InvalidDirectionError.html "enum bevy::math::InvalidDirectionError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#300)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Dir2](../../prelude/struct.Dir2.html "struct bevy::prelude::Dir2"), <[Dir2](../../prelude/struct.Dir2.html "struct bevy::prelude::Dir2") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [type\_path](../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [short\_type\_path](../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [type\_ident](../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [crate\_name](../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [module\_path](../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### fn [type\_info](../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#5)

### impl [Vec2Swizzles](../../prelude/trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#6)

#### type [Vec3](../../prelude/trait.Vec2Swizzles.html#associatedtype.Vec3) = [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#8)

#### type [Vec4](../../prelude/trait.Vec2Swizzles.html#associatedtype.Vec4) = [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#11)

#### fn [xx](../../prelude/trait.Vec2Swizzles.html#tymethod.xx)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#19)

#### fn [yx](../../prelude/trait.Vec2Swizzles.html#tymethod.yx)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#27)

#### fn [yy](../../prelude/trait.Vec2Swizzles.html#tymethod.yy)(self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#35)

#### fn [xxx](../../prelude/trait.Vec2Swizzles.html#tymethod.xxx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#40)

#### fn [xxy](../../prelude/trait.Vec2Swizzles.html#tymethod.xxy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#45)

#### fn [xyx](../../prelude/trait.Vec2Swizzles.html#tymethod.xyx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#50)

#### fn [xyy](../../prelude/trait.Vec2Swizzles.html#tymethod.xyy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#55)

#### fn [yxx](../../prelude/trait.Vec2Swizzles.html#tymethod.yxx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#60)

#### fn [yxy](../../prelude/trait.Vec2Swizzles.html#tymethod.yxy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#65)

#### fn [yyx](../../prelude/trait.Vec2Swizzles.html#tymethod.yyx)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#70)

#### fn [yyy](../../prelude/trait.Vec2Swizzles.html#tymethod.yyy)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#75)

#### fn [xxxx](../../prelude/trait.Vec2Swizzles.html#tymethod.xxxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#80)

#### fn [xxxy](../../prelude/trait.Vec2Swizzles.html#tymethod.xxxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#85)

#### fn [xxyx](../../prelude/trait.Vec2Swizzles.html#tymethod.xxyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#90)

#### fn [xxyy](../../prelude/trait.Vec2Swizzles.html#tymethod.xxyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#95)

#### fn [xyxx](../../prelude/trait.Vec2Swizzles.html#tymethod.xyxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#100)

#### fn [xyxy](../../prelude/trait.Vec2Swizzles.html#tymethod.xyxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#105)

#### fn [xyyx](../../prelude/trait.Vec2Swizzles.html#tymethod.xyyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#110)

#### fn [xyyy](../../prelude/trait.Vec2Swizzles.html#tymethod.xyyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#115)

#### fn [yxxx](../../prelude/trait.Vec2Swizzles.html#tymethod.yxxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#120)

#### fn [yxxy](../../prelude/trait.Vec2Swizzles.html#tymethod.yxxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#125)

#### fn [yxyx](../../prelude/trait.Vec2Swizzles.html#tymethod.yxyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#130)

#### fn [yxyy](../../prelude/trait.Vec2Swizzles.html#tymethod.yxyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#135)

#### fn [yyxx](../../prelude/trait.Vec2Swizzles.html#tymethod.yyxx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#140)

#### fn [yyxy](../../prelude/trait.Vec2Swizzles.html#tymethod.yyxy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#145)

#### fn [yyyx](../../prelude/trait.Vec2Swizzles.html#tymethod.yyyx)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#150)

#### fn [yyyy](../../prelude/trait.Vec2Swizzles.html#tymethod.yyyy)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#10)

#### fn [xy](../../prelude/trait.Vec2Swizzles.html#method.xy)(self) -> Self

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#75)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#77)

#### const [ZERO](../trait.VectorSpace.html#associatedconstant.ZERO): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2") = Vec2::ZERO

The zero vector, which is the identity of addition for the vector space type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#76)

#### type [Scalar](../trait.VectorSpace.html#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The scalar type of this vector space.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#55)

#### fn [lerp](../trait.VectorSpace.html#method.lerp)(self, rhs: Self, t: Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")) -> Self

Perform vector space linear interpolation between this element and another, based on the parameter `t`. When `t` is `0`, `self` is recovered. When `t` is `1`, `rhs` is recovered. [Read more](../trait.VectorSpace.html#method.lerp)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [WriteInto](../../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [AsRefVectorParts](../../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](../../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

#### fn [write\_into](../../render/render_resource/encase/internal/trait.WriteInto.html#tymethod.write_into)<B>(&self, writer: &mut [Writer](../../render/render_resource/encase/internal/struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](../../render/render_resource/encase/internal/trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#21)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}