[bevy](../index.html)::[gizmos](index.html)

# Struct GizmoAsset 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#326)

```rust
pub struct GizmoAsset { /* private fields */ }
```

A collection of gizmos.

Has the same gizmo drawing API as [`Gizmos`](../prelude/struct.Gizmos.html "struct bevy::prelude::Gizmos").

## Implementations

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#332)

### impl [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#334)

#### pub fn [buffer](#method.buffer)(&self) -> &[GizmoBuffer](gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<[ErasedGizmoConfigGroup](config/struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup"), [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

A reference to the gizmo’s vertex buffer.

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#339)

### impl [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#341)

#### pub fn [new](#method.new)() -> [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

Create a new [`GizmoAsset`](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset").

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/gizmos/3d\_gizmos.rs ([line 29](../../src/3d_gizmos/3d_gizmos.rs.html#29))

```rust
23fn setup(
24    mut commands: Commands,
25    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
26    mut meshes: ResMut<Assets<Mesh>>,
27    mut materials: ResMut<Assets<StandardMaterial>>,
28) {
29    let mut gizmo = GizmoAsset::new();
30
31    // When drawing a lot of static lines a Gizmo component can have
32    // far better performance than the Gizmos system parameter,
33    // but the system parameter will perform better for smaller lines that update often.
34
35    // A sphere made out of 30_000 lines!
36    gizmo
37        .sphere(Isometry3d::IDENTITY, 0.5, CRIMSON)
38        .resolution(30_000 / 3);
39
40    commands.spawn((
41        Gizmo {
42            handle: gizmo_assets.add(gizmo),
43            line_config: GizmoLineConfig {
44                width: 5.,
45                ..default()
46            },
47            ..default()
48        },
49        Transform::from_xyz(4., 1., 0.),
50    ));
51
52    commands.spawn((
53        Camera3d::default(),
54        Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
55        FreeCamera::default(),
56    ));
57    // plane
58    commands.spawn((
59        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
60        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
61    ));
62    // cube
63    commands.spawn((
64        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
65        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
66        Transform::from_xyz(0.0, 0.5, 0.0),
67    ));
68    // light
69    commands.spawn((
70        PointLight {
71            shadow_maps_enabled: true,
72            ..default()
73        },
74        Transform::from_xyz(4.0, 8.0, 4.0),
75    ));
76
77    // example instructions
78    commands.spawn((
79        Text::new(
80            "Press 'T' to toggle drawing gizmos on top of everything else in the scene\n\
81            Press 'P' to toggle perspective for line gizmos\n\
82            Hold 'Left' or 'Right' to change the line width of straight gizmos\n\
83            Hold 'Up' or 'Down' to change the line width of round gizmos\n\
84            Press '1' or '2' to toggle the visibility of straight gizmos or round gizmos\n\
85            Press 'B' to show all AABB boxes\n\
86            Press 'U' or 'I' to cycle through line styles for straight or round gizmos\n\
87            Press 'J' or 'K' to cycle through line joins for straight or round gizmos\n\
88            Press 'Spacebar' to toggle pause",
89        ),
90        Node {
91            position_type: PositionType::Absolute,
92            top: px(12),
93            left: px(12),
94            ..default()
95        },
96    ));
97}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#349)

#### pub fn [config\_typeid](#method.config_typeid)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

The type of the gizmo’s configuration group.

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [GizmoBuffer](gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<[ErasedGizmoConfigGroup](config/struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup"), [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/arcs.rs.html#46-52)

#### pub fn [arc\_2d](#method.arc_2d)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>, arc\_angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), radius: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [Arc2dBuilder](arcs/struct.Arc2dBuilder.html "struct bevy::gizmos::arcs::Arc2dBuilder")<'\_, Config, Clear>

Draw an arc, which is a part of the circumference of a circle, in 2D.

##### Arguments

*   `isometry` defines the translation and rotation of the arc.
    *   the translation specifies the center of the arc
    *   the rotation is counter-clockwise starting from `Vec2::Y`
*   `arc_angle` sets the length of this arc, in radians.
*   `radius` controls the distance from `position` to this arc, and thus its curvature.
*   `color` sets the color to draw the arc.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.arc_2d(Isometry2d::IDENTITY, FRAC_PI_4, 1., GREEN);

    // Arcs have 32 line-segments by default.
    // You may want to increase this for larger arcs.
    gizmos
        .arc_2d(Isometry2d::IDENTITY, FRAC_PI_4, 5., RED)
        .resolution(64);
}
```

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([lines 99-104](../../src/2d_gizmos/2d_gizmos.rs.html#99-104))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/arcs.rs.html#168-174)

#### pub fn [arc\_3d](#method.arc_3d)( &mut self, angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), radius: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [Arc3dBuilder](arcs/struct.Arc3dBuilder.html "struct bevy::gizmos::arcs::Arc3dBuilder")<'\_, Config, Clear>

Draw an arc, which is a part of the circumference of a circle, in 3D. For default values this is drawing a standard arc. A standard arc is defined as

*   an arc with a center at `Vec3::ZERO`
*   starting at `Vec3::X`
*   embedded in the XZ plane
*   rotates counterclockwise

##### Arguments

*   `angle`: sets how much of a circle circumference is passed, e.g. PI is half a circle. This value should be in the range (-2 \* PI..=2 \* PI)
*   `radius`: distance between the arc and its center point
*   `isometry` defines the translation and rotation of the arc.
    *   the translation specifies the center of the arc
    *   the rotation is counter-clockwise starting from `Vec3::Y`
*   `color`: color of the arc

##### Builder methods

The resolution of the arc (i.e. the level of detail) can be adjusted with the `.resolution(...)` method.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    // rotation rotates normal to point in the direction of `Vec3::NEG_ONE`
    let rotation = Quat::from_rotation_arc(Vec3::Y, Vec3::NEG_ONE.normalize());

    gizmos
       .arc_3d(
         270.0_f32.to_radians(),
         0.25,
         Isometry3d::new(Vec3::ONE, rotation),
         ORANGE
         )
         .resolution(100);
}
```

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/gizmos/3d\_gizmos.rs ([lines 176-184](../../src/3d_gizmos/3d_gizmos.rs.html#176-184))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/arcs.rs.html#223-229)

#### pub fn [short\_arc\_3d\_between](#method.short_arc_3d_between)( &mut self, center: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), from: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), to: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [Arc3dBuilder](arcs/struct.Arc3dBuilder.html "struct bevy::gizmos::arcs::Arc3dBuilder")<'\_, Config, Clear>

Draws the shortest arc between two points (`from` and `to`) relative to a specified `center` point.

##### Arguments

*   `center`: The center point around which the arc is drawn.
*   `from`: The starting point of the arc.
*   `to`: The ending point of the arc.
*   `color`: color of the arc

##### Builder methods

The resolution of the arc (i.e. the level of detail) can be adjusted with the `.resolution(...)` method.

##### Examples

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.short_arc_3d_between(
       Vec3::ONE,
       Vec3::ONE + Vec3::NEG_ONE,
       Vec3::ZERO,
       ORANGE
       )
       .resolution(100);
}
```

##### Notes

*   This method assumes that the points `from` and `to` are distinct from `center`. If one of the points is coincident with `center`, nothing is rendered.
*   The arc is drawn as a portion of a circle with a radius equal to the distance from the `center` to `from`. If the distance from `center` to `to` is not equal to the radius, then the results will behave as if this were the case

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/arcs.rs.html#269-275)

#### pub fn [long\_arc\_3d\_between](#method.long_arc_3d_between)( &mut self, center: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), from: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), to: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [Arc3dBuilder](arcs/struct.Arc3dBuilder.html "struct bevy::gizmos::arcs::Arc3dBuilder")<'\_, Config, Clear>

Draws the longest arc between two points (`from` and `to`) relative to a specified `center` point.

##### Arguments

*   `center`: The center point around which the arc is drawn.
*   `from`: The starting point of the arc.
*   `to`: The ending point of the arc.
*   `color`: color of the arc

##### Builder methods

The resolution of the arc (i.e. the level of detail) can be adjusted with the `.resolution(...)` method.

##### Examples

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.long_arc_3d_between(
       Vec3::ONE,
       Vec3::ONE + Vec3::NEG_ONE,
       Vec3::ZERO,
       ORANGE
       )
       .resolution(100);
}
```

##### Notes

*   This method assumes that the points `from` and `to` are distinct from `center`. If one of the points is coincident with `center`, nothing is rendered.
*   The arc is drawn as a portion of a circle with a radius equal to the distance from the `center` to `from`. If the distance from `center` to `to` is not equal to the radius, then the results will behave as if this were the case.

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/arcs.rs.html#356-362)

#### pub fn [short\_arc\_2d\_between](#method.short_arc_2d_between)( &mut self, center: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), from: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), to: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [Arc2dBuilder](arcs/struct.Arc2dBuilder.html "struct bevy::gizmos::arcs::Arc2dBuilder")<'\_, Config, Clear>

Draws the shortest arc between two points (`from` and `to`) relative to a specified `center` point.

##### Arguments

*   `center`: The center point around which the arc is drawn.
*   `from`: The starting point of the arc.
*   `to`: The ending point of the arc.
*   `color`: color of the arc

##### Builder methods

The resolution of the arc (i.e. the level of detail) can be adjusted with the `.resolution(...)` method.

##### Examples

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.short_arc_2d_between(
       Vec2::ZERO,
       Vec2::X,
       Vec2::Y,
       ORANGE
       )
       .resolution(100);
}
```

##### Notes

*   This method assumes that the points `from` and `to` are distinct from `center`. If one of the points is coincident with `center`, nothing is rendered.
*   The arc is drawn as a portion of a circle with a radius equal to the distance from the `center` to `from`. If the distance from `center` to `to` is not equal to the radius, then the results will behave as if this were the case

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([line 107](../../src/2d_gizmos/2d_gizmos.rs.html#107))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/arcs.rs.html#402-408)

#### pub fn [long\_arc\_2d\_between](#method.long_arc_2d_between)( &mut self, center: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), from: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), to: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [Arc2dBuilder](arcs/struct.Arc2dBuilder.html "struct bevy::gizmos::arcs::Arc2dBuilder")<'\_, Config, Clear>

Draws the longest arc between two points (`from` and `to`) relative to a specified `center` point.

##### Arguments

*   `center`: The center point around which the arc is drawn.
*   `from`: The starting point of the arc.
*   `to`: The ending point of the arc.
*   `color`: color of the arc

##### Builder methods

The resolution of the arc (i.e. the level of detail) can be adjusted with the `.resolution(...)` method.

##### Examples

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.long_arc_2d_between(
       Vec2::ZERO,
       Vec2::X,
       Vec2::Y,
       ORANGE
       )
       .resolution(100);
}
```

##### Notes

*   This method assumes that the points `from` and `to` are distinct from `center`. If one of the points is coincident with `center`, nothing is rendered.
*   The arc is drawn as a portion of a circle with a radius equal to the distance from the `center` to `from`. If the distance from `center` to `to` is not equal to the radius, then the results will behave as if this were the case.

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([line 106](../../src/2d_gizmos/2d_gizmos.rs.html#106))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/arrows.rs.html#121-126)

#### pub fn [arrow](#method.arrow)( &mut self, start: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), end: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [ArrowBuilder](arrows/struct.ArrowBuilder.html "struct bevy::gizmos::arrows::ArrowBuilder")<'\_, Config, Clear>

Draw an arrow in 3D, from `start` to `end`. Has four tips for convenient viewing from any direction.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.arrow(Vec3::ZERO, Vec3::ONE, GREEN);
}
```

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/transforms/align.rs ([line 125](../../src/align/align.rs.html#125))

```rust
122fn draw_ship_axes(mut gizmos: Gizmos, ship_transform: Single<&Transform, With<Ship>>) {
123    // Local Z-axis arrow, negative direction
124    let z_ends = arrow_ends(*ship_transform, Vec3::NEG_Z, 1.5);
125    gizmos.arrow(z_ends.0, z_ends.1, RED);
126
127    // local X-axis arrow
128    let x_ends = arrow_ends(*ship_transform, Vec3::X, 1.5);
129    gizmos.arrow(x_ends.0, x_ends.1, Color::srgb(0.65, 0., 0.));
130}
131
132// Draw the randomly generated axes
133fn draw_random_axes(mut gizmos: Gizmos, random_axes: Single<&RandomAxes>) {
134    let RandomAxes(v1, v2) = *random_axes;
135    gizmos.arrow(Vec3::ZERO, 1.5 * *v1, WHITE);
136    gizmos.arrow(Vec3::ZERO, 1.5 * *v2, GRAY);
137}
```

Hide additional examples

examples/picking/mesh\_picking.rs ([line 181](../../src/mesh_picking/mesh_picking.rs.html#181))

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

examples/ui/widgets/viewport\_node.rs ([line 125](../../src/viewport_node/viewport_node.rs.html#125))

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

examples/picking/custom\_hit\_data.rs ([lines 187-191](../../src/custom_hit_data/custom_hit_data.rs.html#187-191))

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

examples/gizmos/3d\_gizmos.rs ([line 198](../../src/3d_gizmos/3d_gizmos.rs.html#198))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/arrows.rs.html#150-155)

#### pub fn [arrow\_2d](#method.arrow_2d)( &mut self, start: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), end: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [ArrowBuilder](arrows/struct.ArrowBuilder.html "struct bevy::gizmos::arrows::ArrowBuilder")<'\_, Config, Clear>

Draw an arrow in 2D (on the xy plane), from `start` to `end`.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.arrow_2d(Vec2::ZERO, Vec2::X, GREEN);
}
```

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/math/cubic\_splines.rs ([line 194](../../src/cubic_splines/cubic_splines.rs.html#194))

```rust
185fn draw_control_points(
186    control_points: Res<ControlPoints>,
187    spline_mode: Res<SplineMode>,
188    mut gizmos: Gizmos,
189) {
190    for &(point, tangent) in &control_points.points_and_tangents {
191        gizmos.circle_2d(point, 10.0, Color::srgb(0.0, 1.0, 0.0));
192
193        if matches!(*spline_mode, SplineMode::Hermite) {
194            gizmos.arrow_2d(point, point + tangent, Color::srgb(1.0, 0.0, 0.0));
195        }
196    }
197}
198
199/// Helper function for generating a [`Curve`] from [control points] and selected modes.
200///
201/// [control points]: ControlPoints
202fn form_curve(
203    control_points: &ControlPoints,
204    spline_mode: SplineMode,
205    cycling_mode: CyclingMode,
206) -> Curve {
207    let (points, tangents): (Vec<_>, Vec<_>) =
208        control_points.points_and_tangents.iter().copied().unzip();
209
210    match spline_mode {
211        SplineMode::Hermite => {
212            let spline = CubicHermite::new(points, tangents);
213            Curve(match cycling_mode {
214                CyclingMode::NotCyclic => spline.to_curve().ok(),
215                CyclingMode::Cyclic => spline.to_curve_cyclic().ok(),
216            })
217        }
218        SplineMode::Cardinal => {
219            let spline = CubicCardinalSpline::new_catmull_rom(points);
220            Curve(match cycling_mode {
221                CyclingMode::NotCyclic => spline.to_curve().ok(),
222                CyclingMode::Cyclic => spline.to_curve_cyclic().ok(),
223            })
224        }
225        SplineMode::B => {
226            let spline = CubicBSpline::new(points);
227            Curve(match cycling_mode {
228                CyclingMode::NotCyclic => spline.to_curve().ok(),
229                CyclingMode::Cyclic => spline.to_curve_cyclic().ok(),
230            })
231        }
232    }
233}
234
235// --------------------
236// Text-related Components and Systems
237// --------------------
238
239/// Marker component for the text node that displays the current [`SplineMode`].
240#[derive(Component)]
241struct SplineModeText;
242
243/// Marker component for the text node that displays the current [`CyclingMode`].
244#[derive(Component)]
245struct CyclingModeText;
246
247fn update_spline_mode_text(
248    spline_mode: Res<SplineMode>,
249    mut spline_mode_text: Query<&mut Text, With<SplineModeText>>,
250) {
251    if !spline_mode.is_changed() {
252        return;
253    }
254
255    let new_text = format!("Spline: {}", *spline_mode);
256
257    for mut spline_mode_text in spline_mode_text.iter_mut() {
258        (**spline_mode_text).clone_from(&new_text);
259    }
260}
261
262fn update_cycling_mode_text(
263    cycling_mode: Res<CyclingMode>,
264    mut cycling_mode_text: Query<&mut Text, With<CyclingModeText>>,
265) {
266    if !cycling_mode.is_changed() {
267        return;
268    }
269
270    let new_text = format!("{}", *cycling_mode);
271
272    for mut cycling_mode_text in cycling_mode_text.iter_mut() {
273        (**cycling_mode_text).clone_from(&new_text);
274    }
275}
276
277// -----------------------------------
278// Input-related Resources and Systems
279// -----------------------------------
280
281/// A small state machine which tracks a click-and-drag motion used to create new control points.
282///
283/// When the user is not doing a click-and-drag motion, the `start` field is `None`. When the user
284/// presses the left mouse button, the location of that press is temporarily stored in the field.
285#[derive(Clone, Default, Resource)]
286struct MouseEditMove {
287    start: Option<Vec2>,
288}
289
290/// The current mouse position, if known.
291#[derive(Clone, Default, Resource)]
292struct MousePosition(Option<Vec2>);
293
294/// Update the current cursor position and track it in the [`MousePosition`] resource.
295fn handle_mouse_move(
296    mut cursor_moved_reader: MessageReader<CursorMoved>,
297    mut mouse_position: ResMut<MousePosition>,
298) {
299    if let Some(cursor_moved) = cursor_moved_reader.read().last() {
300        mouse_position.0 = Some(cursor_moved.position);
301    }
302}
303
304/// This system handles updating the [`MouseEditMove`] resource, orchestrating the logical part
305/// of the click-and-drag motion which actually creates new control points.
306fn handle_mouse_press(
307    mut mouse_button_input_reader: MessageReader<MouseButtonInput>,
308    mouse_position: Res<MousePosition>,
309    mut edit_move: ResMut<MouseEditMove>,
310    mut control_points: ResMut<ControlPoints>,
311    camera: Single<(&Camera, &GlobalTransform)>,
312) {
313    let Some(mouse_pos) = mouse_position.0 else {
314        return;
315    };
316
317    // Handle click and drag behavior
318    for mouse_button_input in mouse_button_input_reader.read() {
319        if mouse_button_input.button != MouseButton::Left {
320            continue;
321        }
322
323        match mouse_button_input.state {
324            ButtonState::Pressed => {
325                if edit_move.start.is_some() {
326                    // If the edit move already has a start, press event should do nothing.
327                    continue;
328                }
329                // This press represents the start of the edit move.
330                edit_move.start = Some(mouse_pos);
331            }
332
333            ButtonState::Released => {
334                // Release is only meaningful if we started an edit move.
335                let Some(start) = edit_move.start else {
336                    continue;
337                };
338
339                let (camera, camera_transform) = *camera;
340
341                // Convert the starting point and end point (current mouse pos) into world coords:
342                let Ok(point) = camera.viewport_to_world_2d(camera_transform, start) else {
343                    continue;
344                };
345                let Ok(end_point) = camera.viewport_to_world_2d(camera_transform, mouse_pos) else {
346                    continue;
347                };
348                let tangent = end_point - point;
349
350                // The start of the click-and-drag motion represents the point to add,
351                // while the difference with the current position represents the tangent.
352                control_points.points_and_tangents.push((point, tangent));
353
354                // Reset the edit move since we've consumed it.
355                edit_move.start = None;
356            }
357        }
358    }
359}
360
361/// This system handles drawing the "preview" control point based on the state of [`MouseEditMove`].
362fn draw_edit_move(
363    edit_move: Res<MouseEditMove>,
364    mouse_position: Res<MousePosition>,
365    mut gizmos: Gizmos,
366    camera: Single<(&Camera, &GlobalTransform)>,
367) {
368    let Some(start) = edit_move.start else {
369        return;
370    };
371    let Some(mouse_pos) = mouse_position.0 else {
372        return;
373    };
374
375    let (camera, camera_transform) = *camera;
376
377    // Resources store data in viewport coordinates, so we need to convert to world coordinates
378    // to display them:
379    let Ok(start) = camera.viewport_to_world_2d(camera_transform, start) else {
380        return;
381    };
382    let Ok(end) = camera.viewport_to_world_2d(camera_transform, mouse_pos) else {
383        return;
384    };
385
386    gizmos.circle_2d(start, 10.0, Color::srgb(0.0, 1.0, 0.7));
387    gizmos.circle_2d(start, 7.0, Color::srgb(0.0, 1.0, 0.7));
388    gizmos.arrow_2d(start, end, Color::srgb(1.0, 0.0, 0.7));
389}
```

Hide additional examples

examples/gizmos/2d\_gizmos.rs ([lines 109-113](../../src/2d_gizmos/2d_gizmos.rs.html#109-113))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/arrows.rs.html#185)

#### pub fn [axes](#method.axes)(&mut self, transform: impl [TransformPoint](../prelude/trait.TransformPoint.html "trait bevy::prelude::TransformPoint"), base\_length: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Draw a set of axes local to the given transform (`transform`), with length scaled by a factor of `base_length`.

##### Example

```rust
fn draw_axes(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<MyComponent>>,
) {
    for &transform in &query {
        gizmos.axes(transform, 1.);
    }
}
```

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/testbed/3d.rs ([line 498](../../src/testbed_3d/3d.rs.html#498))

```rust
497    pub fn draw_gizmos(mut gizmos: Gizmos) {
498        gizmos.axes(Transform::IDENTITY, 1.0);
499    }
```

Hide additional examples

examples/gizmos/axes.rs ([line 102](../../src/axes/axes.rs.html#102))

```rust
99fn draw_axes(mut gizmos: Gizmos, query: Query<(&Transform, &Aabb), With<ShowAxes>>) {
100    for (&transform, &aabb) in &query {
101        let length = aabb.half_extents.length();
102        gizmos.axes(transform, length);
103    }
104}
```

examples/animation/custom\_skinned\_mesh.rs ([line 237](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#237))

```rust
192fn joint_animation(
193    time: Res<Time>,
194    mut query: Query<(&mut Transform, &AnimatedJoint)>,
195    mut gizmos: Gizmos,
196) {
197    for (mut transform, animated_joint) in &mut query {
198        match animated_joint.0 {
199            -5 => {
200                transform.rotation =
201                    Quat::from_rotation_x(FRAC_PI_2 * ops::sin(time.elapsed_secs()));
202            }
203            -4 => {
204                transform.rotation =
205                    Quat::from_rotation_y(FRAC_PI_2 * ops::sin(time.elapsed_secs()));
206            }
207            -3 => {
208                transform.rotation =
209                    Quat::from_rotation_z(FRAC_PI_2 * ops::sin(time.elapsed_secs()));
210            }
211            -2 => {
212                transform.scale.x = ops::sin(time.elapsed_secs()) + 1.0;
213            }
214            -1 => {
215                transform.scale.y = ops::sin(time.elapsed_secs()) + 1.0;
216            }
217            0 => {
218                transform.translation.x = 0.5 * ops::sin(time.elapsed_secs());
219                transform.translation.y = ops::cos(time.elapsed_secs());
220            }
221            1 => {
222                transform.translation.y = ops::sin(time.elapsed_secs());
223                transform.translation.z = ops::cos(time.elapsed_secs());
224            }
225            2 => {
226                transform.translation.x = ops::sin(time.elapsed_secs());
227            }
228            3 => {
229                transform.translation.y = ops::sin(time.elapsed_secs());
230                transform.scale.x = ops::sin(time.elapsed_secs()) + 1.0;
231            }
232            _ => (),
233        }
234        // Show transform
235        let mut axis = *transform;
236        axis.translation.x += animated_joint.0 as f32 * 1.5;
237        gizmos.axes(axis, 1.0);
238    }
239}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/arrows.rs.html#216)

#### pub fn [axes\_2d](#method.axes_2d)(&mut self, transform: impl [TransformPoint](../prelude/trait.TransformPoint.html "trait bevy::prelude::TransformPoint"), base\_length: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Draw a set of axes local to the given transform (`transform`), with length scaled by a factor of `base_length`.

##### Example

```rust
fn draw_axes_2d(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<AxesComponent>>,
) {
    for &transform in &query {
        gizmos.axes_2d(transform, 1.);
    }
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/circles.rs.html#50-55)

#### pub fn [ellipse](#method.ellipse)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, half\_size: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [EllipseBuilder](circles/struct.EllipseBuilder.html "struct bevy::gizmos::circles::EllipseBuilder")<'\_, Config, Clear>

Draw an ellipse in 3D with the given `isometry` applied.

If `isometry == Isometry3d::IDENTITY` then

*   the center is at `Vec3::ZERO`
*   the `half_sizes` are aligned with the `Vec3::X` and `Vec3::Y` axes.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.ellipse(Isometry3d::IDENTITY, Vec2::new(1., 2.), GREEN);

    // Ellipses have 32 line-segments by default.
    // You may want to increase this for larger ellipses.
    gizmos
        .ellipse(Isometry3d::IDENTITY, Vec2::new(5., 1.), RED)
        .resolution(64);
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/circles.rs.html#89-94)

#### pub fn [ellipse\_2d](#method.ellipse_2d)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>, half\_size: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [Ellipse2dBuilder](circles/struct.Ellipse2dBuilder.html "struct bevy::gizmos::circles::Ellipse2dBuilder")<'\_, Config, Clear>

Draw an ellipse in 2D with the given `isometry` applied.

If `isometry == Isometry2d::IDENTITY` then

*   the center is at `Vec2::ZERO`
*   the `half_sizes` are aligned with the `Vec2::X` and `Vec2::Y` axes.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.ellipse_2d(Isometry2d::from_rotation(Rot2::degrees(180.0)), Vec2::new(2., 1.), GREEN);

    // Ellipses have 32 line-segments by default.
    // You may want to increase this for larger ellipses.
    gizmos
        .ellipse_2d(Isometry2d::from_rotation(Rot2::degrees(180.0)), Vec2::new(5., 1.), RED)
        .resolution(64);
}
```

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([lines 91-95](../../src/2d_gizmos/2d_gizmos.rs.html#91-95))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/circles.rs.html#128-133)

#### pub fn [circle](#method.circle)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, radius: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [EllipseBuilder](circles/struct.EllipseBuilder.html "struct bevy::gizmos::circles::EllipseBuilder")<'\_, Config, Clear>

Draw a circle in 3D with the given `isometry` applied.

If `isometry == Isometry3d::IDENTITY` then

*   the center is at `Vec3::ZERO`
*   the radius is aligned with the `Vec3::X` and `Vec3::Y` axes.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.circle(Isometry3d::IDENTITY, 1., GREEN);

    // Circles have 32 line-segments by default.
    // You may want to increase this for larger circles.
    gizmos
        .circle(Isometry3d::IDENTITY, 5., RED)
        .resolution(64);
}
```

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/3d/3d\_viewport\_to\_world.rs ([lines 28-35](../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#28-35))

```rust
13fn draw_cursor(
14    camera_query: Single<(&Camera, &GlobalTransform)>,
15    ground: Single<&GlobalTransform, With<Ground>>,
16    window: Single<&Window>,
17    mut gizmos: Gizmos,
18) {
19    let (camera, camera_transform) = *camera_query;
20
21    if let Some(cursor_position) = window.cursor_position()
22        // Calculate a ray pointing from the camera into the world based on the cursor's position.
23        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
24        // Calculate if and where the ray is hitting the ground plane.
25        && let Some(point) = ray.plane_intersection_point(ground.translation(), InfinitePlane3d::new(ground.up()))
26    {
27        // Draw a circle just above the ground plane at that position.
28        gizmos.circle(
29            Isometry3d::new(
30                point + ground.up() * 0.01,
31                Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
32            ),
33            0.2,
34            Color::WHITE,
35        );
36    }
37}
```

Hide additional examples

examples/gizmos/3d\_gizmos.rs ([line 188](../../src/3d_gizmos/3d_gizmos.rs.html#188))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/circles.rs.html#167-172)

#### pub fn [circle\_2d](#method.circle_2d)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>, radius: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [Ellipse2dBuilder](circles/struct.Ellipse2dBuilder.html "struct bevy::gizmos::circles::Ellipse2dBuilder")<'\_, Config, Clear>

Draw a circle in 2D with the given `isometry` applied.

If `isometry == Isometry2d::IDENTITY` then

*   the center is at `Vec2::ZERO`
*   the radius is aligned with the `Vec2::X` and `Vec2::Y` axes.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.circle_2d(Isometry2d::IDENTITY, 1., GREEN);

    // Circles have 32 line-segments by default.
    // You may want to increase this for larger circles.
    gizmos
        .circle_2d(Isometry2d::IDENTITY, 5., RED)
        .resolution(64);
}
```

##### [Examples found in repository](#scraped-examples-10)[?](../../scrape-examples-help.html)

examples/ecs/observers.rs ([lines 181-185](../../src/observers/observers.rs.html#181-185))

```rust
179fn draw_shapes(mut gizmos: Gizmos, mines: Query<&Mine>) {
180    for mine in &mines {
181        gizmos.circle_2d(
182            mine.pos,
183            mine.size,
184            Color::hsl((mine.size - 4.0) / 16.0 * 360.0, 1.0, 0.8),
185        );
186    }
187}
```

Hide additional examples

examples/math/cubic\_splines.rs ([line 191](../../src/cubic_splines/cubic_splines.rs.html#191))

```rust
185fn draw_control_points(
186    control_points: Res<ControlPoints>,
187    spline_mode: Res<SplineMode>,
188    mut gizmos: Gizmos,
189) {
190    for &(point, tangent) in &control_points.points_and_tangents {
191        gizmos.circle_2d(point, 10.0, Color::srgb(0.0, 1.0, 0.0));
192
193        if matches!(*spline_mode, SplineMode::Hermite) {
194            gizmos.arrow_2d(point, point + tangent, Color::srgb(1.0, 0.0, 0.0));
195        }
196    }
197}
198
199/// Helper function for generating a [`Curve`] from [control points] and selected modes.
200///
201/// [control points]: ControlPoints
202fn form_curve(
203    control_points: &ControlPoints,
204    spline_mode: SplineMode,
205    cycling_mode: CyclingMode,
206) -> Curve {
207    let (points, tangents): (Vec<_>, Vec<_>) =
208        control_points.points_and_tangents.iter().copied().unzip();
209
210    match spline_mode {
211        SplineMode::Hermite => {
212            let spline = CubicHermite::new(points, tangents);
213            Curve(match cycling_mode {
214                CyclingMode::NotCyclic => spline.to_curve().ok(),
215                CyclingMode::Cyclic => spline.to_curve_cyclic().ok(),
216            })
217        }
218        SplineMode::Cardinal => {
219            let spline = CubicCardinalSpline::new_catmull_rom(points);
220            Curve(match cycling_mode {
221                CyclingMode::NotCyclic => spline.to_curve().ok(),
222                CyclingMode::Cyclic => spline.to_curve_cyclic().ok(),
223            })
224        }
225        SplineMode::B => {
226            let spline = CubicBSpline::new(points);
227            Curve(match cycling_mode {
228                CyclingMode::NotCyclic => spline.to_curve().ok(),
229                CyclingMode::Cyclic => spline.to_curve_cyclic().ok(),
230            })
231        }
232    }
233}
234
235// --------------------
236// Text-related Components and Systems
237// --------------------
238
239/// Marker component for the text node that displays the current [`SplineMode`].
240#[derive(Component)]
241struct SplineModeText;
242
243/// Marker component for the text node that displays the current [`CyclingMode`].
244#[derive(Component)]
245struct CyclingModeText;
246
247fn update_spline_mode_text(
248    spline_mode: Res<SplineMode>,
249    mut spline_mode_text: Query<&mut Text, With<SplineModeText>>,
250) {
251    if !spline_mode.is_changed() {
252        return;
253    }
254
255    let new_text = format!("Spline: {}", *spline_mode);
256
257    for mut spline_mode_text in spline_mode_text.iter_mut() {
258        (**spline_mode_text).clone_from(&new_text);
259    }
260}
261
262fn update_cycling_mode_text(
263    cycling_mode: Res<CyclingMode>,
264    mut cycling_mode_text: Query<&mut Text, With<CyclingModeText>>,
265) {
266    if !cycling_mode.is_changed() {
267        return;
268    }
269
270    let new_text = format!("{}", *cycling_mode);
271
272    for mut cycling_mode_text in cycling_mode_text.iter_mut() {
273        (**cycling_mode_text).clone_from(&new_text);
274    }
275}
276
277// -----------------------------------
278// Input-related Resources and Systems
279// -----------------------------------
280
281/// A small state machine which tracks a click-and-drag motion used to create new control points.
282///
283/// When the user is not doing a click-and-drag motion, the `start` field is `None`. When the user
284/// presses the left mouse button, the location of that press is temporarily stored in the field.
285#[derive(Clone, Default, Resource)]
286struct MouseEditMove {
287    start: Option<Vec2>,
288}
289
290/// The current mouse position, if known.
291#[derive(Clone, Default, Resource)]
292struct MousePosition(Option<Vec2>);
293
294/// Update the current cursor position and track it in the [`MousePosition`] resource.
295fn handle_mouse_move(
296    mut cursor_moved_reader: MessageReader<CursorMoved>,
297    mut mouse_position: ResMut<MousePosition>,
298) {
299    if let Some(cursor_moved) = cursor_moved_reader.read().last() {
300        mouse_position.0 = Some(cursor_moved.position);
301    }
302}
303
304/// This system handles updating the [`MouseEditMove`] resource, orchestrating the logical part
305/// of the click-and-drag motion which actually creates new control points.
306fn handle_mouse_press(
307    mut mouse_button_input_reader: MessageReader<MouseButtonInput>,
308    mouse_position: Res<MousePosition>,
309    mut edit_move: ResMut<MouseEditMove>,
310    mut control_points: ResMut<ControlPoints>,
311    camera: Single<(&Camera, &GlobalTransform)>,
312) {
313    let Some(mouse_pos) = mouse_position.0 else {
314        return;
315    };
316
317    // Handle click and drag behavior
318    for mouse_button_input in mouse_button_input_reader.read() {
319        if mouse_button_input.button != MouseButton::Left {
320            continue;
321        }
322
323        match mouse_button_input.state {
324            ButtonState::Pressed => {
325                if edit_move.start.is_some() {
326                    // If the edit move already has a start, press event should do nothing.
327                    continue;
328                }
329                // This press represents the start of the edit move.
330                edit_move.start = Some(mouse_pos);
331            }
332
333            ButtonState::Released => {
334                // Release is only meaningful if we started an edit move.
335                let Some(start) = edit_move.start else {
336                    continue;
337                };
338
339                let (camera, camera_transform) = *camera;
340
341                // Convert the starting point and end point (current mouse pos) into world coords:
342                let Ok(point) = camera.viewport_to_world_2d(camera_transform, start) else {
343                    continue;
344                };
345                let Ok(end_point) = camera.viewport_to_world_2d(camera_transform, mouse_pos) else {
346                    continue;
347                };
348                let tangent = end_point - point;
349
350                // The start of the click-and-drag motion represents the point to add,
351                // while the difference with the current position represents the tangent.
352                control_points.points_and_tangents.push((point, tangent));
353
354                // Reset the edit move since we've consumed it.
355                edit_move.start = None;
356            }
357        }
358    }
359}
360
361/// This system handles drawing the "preview" control point based on the state of [`MouseEditMove`].
362fn draw_edit_move(
363    edit_move: Res<MouseEditMove>,
364    mouse_position: Res<MousePosition>,
365    mut gizmos: Gizmos,
366    camera: Single<(&Camera, &GlobalTransform)>,
367) {
368    let Some(start) = edit_move.start else {
369        return;
370    };
371    let Some(mouse_pos) = mouse_position.0 else {
372        return;
373    };
374
375    let (camera, camera_transform) = *camera;
376
377    // Resources store data in viewport coordinates, so we need to convert to world coordinates
378    // to display them:
379    let Ok(start) = camera.viewport_to_world_2d(camera_transform, start) else {
380        return;
381    };
382    let Ok(end) = camera.viewport_to_world_2d(camera_transform, mouse_pos) else {
383        return;
384    };
385
386    gizmos.circle_2d(start, 10.0, Color::srgb(0.0, 1.0, 0.7));
387    gizmos.circle_2d(start, 7.0, Color::srgb(0.0, 1.0, 0.7));
388    gizmos.arrow_2d(start, end, Color::srgb(1.0, 0.0, 0.7));
389}
```

examples/math/bounding\_2d.rs ([line 190](../../src/bounding_2d/bounding_2d.rs.html#190))

```rust
182fn render_volumes(mut gizmos: Gizmos, query: Query<(&CurrentVolume, &Intersects)>) {
183    for (volume, intersects) in query.iter() {
184        let color = if **intersects { AQUA } else { ORANGE_RED };
185        match volume {
186            CurrentVolume::Aabb(a) => {
187                gizmos.rect_2d(a.center(), a.half_size() * 2., color);
188            }
189            CurrentVolume::Circle(c) => {
190                gizmos.circle_2d(c.center(), c.radius(), color);
191            }
192        }
193    }
194}
195
196#[derive(Component, Deref, DerefMut, Default)]
197struct Intersects(bool);
198
199const OFFSET_X: f32 = 125.;
200const OFFSET_Y: f32 = 75.;
201
202fn setup(mut commands: Commands) {
203    commands.spawn(Camera2d);
204
205    commands.spawn((
206        Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.),
207        Shape::Circle(Circle::new(45.)),
208        DesiredVolume::Aabb,
209        Intersects::default(),
210    ));
211
212    commands.spawn((
213        Transform::from_xyz(0., OFFSET_Y, 0.),
214        Shape::Rectangle(Rectangle::new(80., 80.)),
215        Spin,
216        DesiredVolume::Circle,
217        Intersects::default(),
218    ));
219
220    commands.spawn((
221        Transform::from_xyz(OFFSET_X, OFFSET_Y, 0.),
222        Shape::Triangle(Triangle2d::new(
223            Vec2::new(-40., -40.),
224            Vec2::new(-20., 40.),
225            Vec2::new(40., 50.),
226        )),
227        Spin,
228        DesiredVolume::Aabb,
229        Intersects::default(),
230    ));
231
232    commands.spawn((
233        Transform::from_xyz(-OFFSET_X, -OFFSET_Y, 0.),
234        Shape::Line(Segment2d::from_direction_and_length(
235            Dir2::from_xy(1., 0.3).unwrap(),
236            90.,
237        )),
238        Spin,
239        DesiredVolume::Circle,
240        Intersects::default(),
241    ));
242
243    commands.spawn((
244        Transform::from_xyz(0., -OFFSET_Y, 0.),
245        Shape::Capsule(Capsule2d::new(25., 50.)),
246        Spin,
247        DesiredVolume::Aabb,
248        Intersects::default(),
249    ));
250
251    commands.spawn((
252        Transform::from_xyz(OFFSET_X, -OFFSET_Y, 0.),
253        Shape::Polygon(RegularPolygon::new(50., 6)),
254        Spin,
255        DesiredVolume::Circle,
256        Intersects::default(),
257    ));
258
259    commands.spawn((
260        Text::default(),
261        Node {
262            position_type: PositionType::Absolute,
263            top: px(12),
264            left: px(12),
265            ..default()
266        },
267    ));
268}
269
270fn draw_filled_circle(gizmos: &mut Gizmos, position: Vec2, color: Srgba) {
271    for r in [1., 2., 3.] {
272        gizmos.circle_2d(position, r, color);
273    }
274}
275
276fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
277    gizmos.line_2d(
278        ray.ray.origin,
279        ray.ray.origin + *ray.ray.direction * ray.max,
280        WHITE,
281    );
282    draw_filled_circle(gizmos, ray.ray.origin, FUCHSIA);
283}
284
285fn get_and_draw_ray(gizmos: &mut Gizmos, time: &Time) -> RayCast2d {
286    let ray = Vec2::new(ops::cos(time.elapsed_secs()), ops::sin(time.elapsed_secs()));
287    let dist = 150. + ops::sin(0.5 * time.elapsed_secs()).abs() * 500.;
288
289    let aabb_ray = Ray2d {
290        origin: ray * 250.,
291        direction: Dir2::new_unchecked(-ray),
292    };
293    let ray_cast = RayCast2d::from_ray(aabb_ray, dist - 20.);
294
295    draw_ray(gizmos, &ray_cast);
296    ray_cast
297}
298
299fn ray_cast_system(
300    mut gizmos: Gizmos,
301    time: Res<Time>,
302    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
303) {
304    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
305
306    for (volume, mut intersects) in volumes.iter_mut() {
307        let toi = match volume {
308            CurrentVolume::Aabb(a) => ray_cast.aabb_intersection_at(a),
309            CurrentVolume::Circle(c) => ray_cast.circle_intersection_at(c),
310        };
311        **intersects = toi.is_some();
312        if let Some(toi) = toi {
313            draw_filled_circle(
314                &mut gizmos,
315                ray_cast.ray.origin + *ray_cast.ray.direction * toi,
316                LIME,
317            );
318        }
319    }
320}
321
322fn aabb_cast_system(
323    mut gizmos: Gizmos,
324    time: Res<Time>,
325    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
326) {
327    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
328    let aabb_cast = AabbCast2d {
329        aabb: Aabb2d::new(Vec2::ZERO, Vec2::splat(15.)),
330        ray: ray_cast,
331    };
332
333    for (volume, mut intersects) in volumes.iter_mut() {
334        let toi = match *volume {
335            CurrentVolume::Aabb(a) => aabb_cast.aabb_collision_at(a),
336            CurrentVolume::Circle(_) => None,
337        };
338
339        **intersects = toi.is_some();
340        if let Some(toi) = toi {
341            gizmos.rect_2d(
342                aabb_cast.ray.ray.origin + *aabb_cast.ray.ray.direction * toi,
343                aabb_cast.aabb.half_size() * 2.,
344                LIME,
345            );
346        }
347    }
348}
349
350fn bounding_circle_cast_system(
351    mut gizmos: Gizmos,
352    time: Res<Time>,
353    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
354) {
355    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
356    let circle_cast = BoundingCircleCast {
357        circle: BoundingCircle::new(Vec2::ZERO, 15.),
358        ray: ray_cast,
359    };
360
361    for (volume, mut intersects) in volumes.iter_mut() {
362        let toi = match *volume {
363            CurrentVolume::Aabb(_) => None,
364            CurrentVolume::Circle(c) => circle_cast.circle_collision_at(c),
365        };
366
367        **intersects = toi.is_some();
368        if let Some(toi) = toi {
369            gizmos.circle_2d(
370                circle_cast.ray.ray.origin + *circle_cast.ray.ray.direction * toi,
371                circle_cast.circle.radius(),
372                LIME,
373            );
374        }
375    }
376}
377
378fn get_intersection_position(time: &Time) -> Vec2 {
379    let x = ops::cos(0.8 * time.elapsed_secs()) * 250.;
380    let y = ops::sin(0.4 * time.elapsed_secs()) * 100.;
381    Vec2::new(x, y)
382}
383
384fn aabb_intersection_system(
385    mut gizmos: Gizmos,
386    time: Res<Time>,
387    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
388) {
389    let center = get_intersection_position(&time);
390    let aabb = Aabb2d::new(center, Vec2::splat(50.));
391    gizmos.rect_2d(center, aabb.half_size() * 2., YELLOW);
392
393    for (volume, mut intersects) in volumes.iter_mut() {
394        let hit = match volume {
395            CurrentVolume::Aabb(a) => aabb.intersects(a),
396            CurrentVolume::Circle(c) => aabb.intersects(c),
397        };
398
399        **intersects = hit;
400    }
401}
402
403fn circle_intersection_system(
404    mut gizmos: Gizmos,
405    time: Res<Time>,
406    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
407) {
408    let center = get_intersection_position(&time);
409    let circle = BoundingCircle::new(center, 50.);
410    gizmos.circle_2d(center, circle.radius(), YELLOW);
411
412    for (volume, mut intersects) in volumes.iter_mut() {
413        let hit = match volume {
414            CurrentVolume::Aabb(a) => circle.intersects(a),
415            CurrentVolume::Circle(c) => circle.intersects(c),
416        };
417
418        **intersects = hit;
419    }
420}
```

examples/2d/mesh2d\_arcs.rs ([line 118](../../src/mesh2d_arcs/mesh2d_arcs.rs.html#118))

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

examples/2d/2d\_viewport\_to\_world.rs ([line 36](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#36))

```rust
22fn draw_cursor(
23    camera_query: Single<(&Camera, &GlobalTransform)>,
24    window: Single<&Window>,
25    mut gizmos: Gizmos,
26) {
27    let (camera, camera_transform) = *camera_query;
28
29    if let Some(cursor_position) = window.cursor_position()
30        // Calculate a world position based on the cursor's position.
31        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
32        // To test Camera::world_to_viewport, convert result back to viewport space and then back to world space.
33        && let Ok(viewport_check) = camera.world_to_viewport(camera_transform, world_pos.extend(0.0))
34        && let Ok(world_check) = camera.viewport_to_world_2d(camera_transform, viewport_check.xy())
35    {
36        gizmos.circle_2d(world_pos, 10., WHITE);
37        // Should be the same as world_pos
38        gizmos.circle_2d(world_check, 8., RED);
39    }
40}
```

examples/testbed/2d.rs ([lines 401-405](../../src/testbed_2d/2d.rs.html#401-405))

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

Additional examples can be found in:  

*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#305)
*   [examples/gizmos/2d\_gizmos.rs](../../src/2d_gizmos/2d_gizmos.rs.html#88)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/circles.rs.html#207-212)

#### pub fn [sphere](#method.sphere)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, radius: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [SphereBuilder](circles/struct.SphereBuilder.html "struct bevy::gizmos::circles::SphereBuilder")<'\_, Config, Clear>

Draw a wireframe sphere in 3D made out of 3 circles around the axes with the given `isometry` applied.

If `isometry == Isometry3d::IDENTITY` then

*   the center is at `Vec3::ZERO`
*   the 3 circles are in the XY, YZ and XZ planes.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.sphere(Isometry3d::IDENTITY, 1., Color::BLACK);

    // Each circle has 32 line-segments by default.
    // You may want to increase this for larger spheres.
    gizmos
        .sphere(Isometry3d::IDENTITY, 5., Color::BLACK)
        .resolution(64);
}
```

##### [Examples found in repository](#scraped-examples-11)[?](../../scrape-examples-help.html)

examples/picking/mesh\_picking.rs ([line 180](../../src/mesh_picking/mesh_picking.rs.html#180))

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

examples/3d/mesh\_ray\_cast.rs ([line 35](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#35))

```rust
23fn bouncing_raycast(
24    mut ray_cast: MeshRayCast,
25    mut gizmos: Gizmos,
26    time: Res<Time>,
27    // The ray map stores rays cast by the cursor
28    ray_map: Res<RayMap>,
29) {
30    // Cast an automatically moving ray and bounce it off of surfaces
31    let t = ops::cos((time.elapsed_secs() - 4.0).max(0.0) * LASER_SPEED) * PI;
32    let ray_pos = Vec3::new(ops::sin(t), ops::cos(3.0 * t) * 0.5, ops::cos(t)) * 0.5;
33    let ray_dir = Dir3::new(-ray_pos).unwrap();
34    let ray = Ray3d::new(ray_pos, ray_dir);
35    gizmos.sphere(ray_pos, 0.1, Color::WHITE);
36    bounce_ray(ray, &mut ray_cast, &mut gizmos, Color::from(css::RED));
37
38    // Cast a ray from the cursor and bounce it off of surfaces
39    for (_, ray) in ray_map.iter() {
40        bounce_ray(*ray, &mut ray_cast, &mut gizmos, Color::from(css::GREEN));
41    }
42}
43
44// Bounces a ray off of surfaces `MAX_BOUNCES` times.
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

examples/math/custom\_primitives.rs ([line 344](../../src/custom_primitives/custom_primitives.rs.html#344))

```rust
322fn bounding_shapes_3d(
323    shapes: Query<&Transform, With<Shape3d>>,
324    mut gizmos: Gizmos,
325    bounding_shape: Res<State<BoundingShape>>,
326) {
327    for transform in shapes.iter() {
328        match bounding_shape.get() {
329            BoundingShape::None => (),
330            BoundingShape::BoundingBox => {
331                // Get the AABB of the extrusion with the rotation and translation of the mesh.
332                let aabb = EXTRUSION.aabb_3d(transform.to_isometry());
333
334                gizmos.primitive_3d(
335                    &Cuboid::from_size(Vec3::from(aabb.half_size()) * 2.),
336                    aabb.center(),
337                    WHITE,
338                );
339            }
340            BoundingShape::BoundingSphere => {
341                // Get the bounding sphere of the extrusion with the rotation and translation of the mesh.
342                let bounding_sphere = EXTRUSION.bounding_sphere(transform.to_isometry());
343
344                gizmos.sphere(bounding_sphere.center(), bounding_sphere.radius(), WHITE);
345            }
346        }
347    }
348}
```

examples/testbed/3d.rs ([line 385](../../src/testbed_3d/3d.rs.html#385))

```rust
379    pub fn draw_gizmos(mut gizmos: Gizmos) {
380        gizmos.cube(
381            Transform::from_translation(Vec3::X * -1.75).with_scale(Vec3::splat(1.25)),
382            RED,
383        );
384        gizmos
385            .sphere(Isometry3d::from_translation(Vec3::X * -3.5), 0.75, GREEN)
386            .resolution(30_000 / 3);
387
388        gizmos.text(
389            Isometry3d::from_translation(Vec3::Y * 1.5),
390            "text gizmo",
391            0.3,
392            Vec2 { x: 0., y: 0. },
393            Color::WHITE,
394        );
395
396        // 3d grids with all variations of outer edges on or off
397        for i in 0..8 {
398            let x = 1.5 * (i % 4) as f32;
399            let y = 1.0 * (0.5 - (i / 4) as f32);
400            let mut grid = gizmos.grid_3d(
401                Isometry3d::from_translation(Vec3::new(x, y, 0.0)),
402                UVec3::new(5, 4, 3),
403                Vec3::splat(0.175),
404                Color::WHITE,
405            );
406            if i & 1 > 0 {
407                grid = grid.outer_edges_x();
408            }
409            if i & 2 > 0 {
410                grid = grid.outer_edges_y();
411            }
412            if i & 4 > 0 {
413                grid.outer_edges_z();
414            }
415        }
416    }
```

examples/gizmos/3d\_gizmos.rs ([line 37](../../src/3d_gizmos/3d_gizmos.rs.html#37))

```rust
23fn setup(
24    mut commands: Commands,
25    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
26    mut meshes: ResMut<Assets<Mesh>>,
27    mut materials: ResMut<Assets<StandardMaterial>>,
28) {
29    let mut gizmo = GizmoAsset::new();
30
31    // When drawing a lot of static lines a Gizmo component can have
32    // far better performance than the Gizmos system parameter,
33    // but the system parameter will perform better for smaller lines that update often.
34
35    // A sphere made out of 30_000 lines!
36    gizmo
37        .sphere(Isometry3d::IDENTITY, 0.5, CRIMSON)
38        .resolution(30_000 / 3);
39
40    commands.spawn((
41        Gizmo {
42            handle: gizmo_assets.add(gizmo),
43            line_config: GizmoLineConfig {
44                width: 5.,
45                ..default()
46            },
47            ..default()
48        },
49        Transform::from_xyz(4., 1., 0.),
50    ));
51
52    commands.spawn((
53        Camera3d::default(),
54        Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
55        FreeCamera::default(),
56    ));
57    // plane
58    commands.spawn((
59        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
60        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
61    ));
62    // cube
63    commands.spawn((
64        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
65        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
66        Transform::from_xyz(0.0, 0.5, 0.0),
67    ));
68    // light
69    commands.spawn((
70        PointLight {
71            shadow_maps_enabled: true,
72            ..default()
73        },
74        Transform::from_xyz(4.0, 8.0, 4.0),
75    ));
76
77    // example instructions
78    commands.spawn((
79        Text::new(
80            "Press 'T' to toggle drawing gizmos on top of everything else in the scene\n\
81            Press 'P' to toggle perspective for line gizmos\n\
82            Hold 'Left' or 'Right' to change the line width of straight gizmos\n\
83            Hold 'Up' or 'Down' to change the line width of round gizmos\n\
84            Press '1' or '2' to toggle the visibility of straight gizmos or round gizmos\n\
85            Press 'B' to show all AABB boxes\n\
86            Press 'U' or 'I' to cycle through line styles for straight or round gizmos\n\
87            Press 'J' or 'K' to cycle through line joins for straight or round gizmos\n\
88            Press 'Spacebar' to toggle pause",
89        ),
90        Node {
91            position_type: PositionType::Absolute,
92            top: px(12),
93            left: px(12),
94            ..default()
95        },
96    ));
97}
98
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/cross.rs.html#32-37)

#### pub fn [cross](#method.cross)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, half\_size: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a cross in 3D with the given `isometry` applied.

If `isometry == Isometry3d::IDENTITY` then

*   the center is at `Vec3::ZERO`
*   the `half_size`s are aligned with the `Vec3::X`, `Vec3::Y` and `Vec3::Z` axes.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.cross(Isometry3d::IDENTITY, 0.5, WHITE);
}
```

##### [Examples found in repository](#scraped-examples-12)[?](../../scrape-examples-help.html)

examples/gizmos/3d\_gizmos.rs ([line 147](../../src/3d_gizmos/3d_gizmos.rs.html#147))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/cross.rs.html#65-70)

#### pub fn [cross\_2d](#method.cross_2d)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>, half\_size: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a cross in 2D with the given `isometry` applied.

If `isometry == Isometry2d::IDENTITY` then

*   the center is at `Vec3::ZERO`
*   the `half_size`s are aligned with the `Vec3::X` and `Vec3::Y` axes.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.cross_2d(Isometry2d::IDENTITY, 0.5, WHITE);
}
```

##### [Examples found in repository](#scraped-examples-13)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([line 70](../../src/2d_gizmos/2d_gizmos.rs.html#70))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/curves.rs.html#42-47)

#### pub fn [curve\_2d](#method.curve_2d)( &mut self, curve\_2d: impl [Curve](../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>, times: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a curve, at the given time points, sampling in 2D.

Samples of time points outside of the curve’s domain will be filtered out and won’t contribute to the rendering. If you wish to render the curve outside of its domain you need to create a new curve with an extended domain.

##### Arguments

*   `curve_2d` some type that implements the [`Curve`](../prelude/trait.Curve.html "trait bevy::prelude::Curve") trait and samples `Vec2`s
*   `times` some iterable type yielding `f32` which will be used for sampling the curve
*   `color` the color of the curve

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    let domain = Interval::UNIT;
    let curve = FunctionCurve::new(domain, |t| Vec2::from(t.sin_cos()));
    gizmos.curve_2d(curve, (0..=100).map(|n| n as f32 / 100.0), RED);
}
```

##### [Examples found in repository](#scraped-examples-14)[?](../../scrape-examples-help.html)

examples/animation/easing\_functions.rs ([lines 162-166](../../src/easing_functions/easing_functions.rs.html#162-166))

```rust
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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/curves.rs.html#77-82)

#### pub fn [curve\_3d](#method.curve_3d)( &mut self, curve\_3d: impl [Curve](../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>, times: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a curve, at the given time points, sampling in 3D.

Samples of time points outside of the curve’s domain will be filtered out and won’t contribute to the rendering. If you wish to render the curve outside of its domain you need to create a new curve with an extended domain.

##### Arguments

*   `curve_3d` some type that implements the [`Curve`](../prelude/trait.Curve.html "trait bevy::prelude::Curve") trait and samples `Vec3`s
*   `times` some iterable type yielding `f32` which will be used for sampling the curve
*   `color` the color of the curve

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    let domain = Interval::UNIT;
    let curve = FunctionCurve::new(domain, |t| {
        let (x,y) = t.sin_cos();
        Vec3::new(x, y, t)
    });
    gizmos.curve_3d(curve, (0..=100).map(|n| n as f32 / 100.0), RED);
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/curves.rs.html#113-118)

#### pub fn [curve\_gradient\_2d](#method.curve_gradient_2d)<C>( &mut self, curve\_2d: impl [Curve](../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>, times\_with\_colors: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), C)>, )

where C: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>,

Draw a curve, at the given time points, sampling in 2D, with a color gradient.

Samples of time points outside of the curve’s domain will be filtered out and won’t contribute to the rendering. If you wish to render the curve outside of its domain you need to create a new curve with an extended domain.

##### Arguments

*   `curve_2d` some type that implements the [`Curve`](../prelude/trait.Curve.html "trait bevy::prelude::Curve") trait and samples `Vec2`s
*   `times_with_colors` some iterable type yielding `f32` which will be used for sampling the curve together with the color at this position

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    let domain = Interval::UNIT;
    let curve = FunctionCurve::new(domain, |t| Vec2::from(t.sin_cos()));
    gizmos.curve_gradient_2d(
        curve,
        (0..=100).map(|n| n as f32 / 100.0)
                 .map(|t| (t, GREEN.mix(&RED, t)))
    );
}
```

##### [Examples found in repository](#scraped-examples-15)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([line 79](../../src/2d_gizmos/2d_gizmos.rs.html#79))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/curves.rs.html#157-162)

#### pub fn [curve\_gradient\_3d](#method.curve_gradient_3d)<C>( &mut self, curve\_3d: impl [Curve](../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>, times\_with\_colors: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), C)>, )

where C: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>,

Draw a curve, at the given time points, sampling in 3D, with a color gradient.

Samples of time points outside of the curve’s domain will be filtered out and won’t contribute to the rendering. If you wish to render the curve outside of its domain you need to create a new curve with an extended domain.

##### Arguments

*   `curve_3d` some type that implements the [`Curve`](../prelude/trait.Curve.html "trait bevy::prelude::Curve") trait and samples `Vec3`s
*   `times_with_colors` some iterable type yielding `f32` which will be used for sampling the curve together with the color at this position

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    let domain = Interval::UNIT;
    let curve = FunctionCurve::new(domain, |t| {
        let (x,y) = t.sin_cos();
        Vec3::new(x, y, t)
    });
    gizmos.curve_gradient_3d(
        curve,
        (0..=100).map(|n| n as f32 / 100.0)
                 .map(|t| (t, GREEN.mix(&RED, t)))
    );
}
```

##### [Examples found in repository](#scraped-examples-16)[?](../../scrape-examples-help.html)

examples/gizmos/3d\_gizmos.rs ([line 158](../../src/3d_gizmos/3d_gizmos.rs.html#158))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#372)

#### pub fn [clear](#method.clear)(&mut self)

Clear all data.

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#380)

#### pub fn [buffer](#method.buffer-1)(&self) -> [GizmoBufferView](gizmos/struct.GizmoBufferView.html "struct bevy::gizmos::gizmos::GizmoBufferView")<'\_>

Read-only view into the buffers data.

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#408)

#### pub fn [line](#method.line)(&mut self, start: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), end: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>)

Draw a line in 3D from `start` to `end`.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::X, GREEN);
}
```

##### [Examples found in repository](#scraped-examples-17)[?](../../scrape-examples-help.html)

examples/stress\_tests/many\_gizmos.rs ([line 65](../../src/many_gizmos/many_gizmos.rs.html#65))

```rust
62fn system(config: Res<Config>, time: Res<Time>, mut draw: Gizmos) {
63    if !config.fancy {
64        for _ in 0..(config.line_count / SYSTEM_COUNT) {
65            draw.line(Vec3::NEG_Y, Vec3::Y, Color::BLACK);
66        }
67    } else {
68        for i in 0..(config.line_count / SYSTEM_COUNT) {
69            let angle = i as f32 / (config.line_count / SYSTEM_COUNT) as f32 * TAU;
70
71            let vector = Vec2::from(ops::sin_cos(angle)).extend(ops::sin(time.elapsed_secs()));
72            let start_color = LinearRgba::rgb(vector.x, vector.z, 0.5);
73            let end_color = LinearRgba::rgb(-vector.z, -vector.y, 0.5);
74
75            draw.line_gradient(vector, -vector, start_color, end_color);
76        }
77    }
78}
```

Hide additional examples

examples/picking/custom\_hit\_data.rs ([line 201](../../src/custom_hit_data/custom_hit_data.rs.html#201))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#429-435)

#### pub fn [line\_gradient](#method.line_gradient)<C>( &mut self, start: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), end: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), start\_color: C, end\_color: C, )

where C: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>,

Draw a line in 3D with a color gradient from `start` to `end`.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.line_gradient(Vec3::ZERO, Vec3::X, GREEN, RED);
}
```

##### [Examples found in repository](#scraped-examples-18)[?](../../scrape-examples-help.html)

examples/stress\_tests/many\_gizmos.rs ([line 75](../../src/many_gizmos/many_gizmos.rs.html#75))

```rust
62fn system(config: Res<Config>, time: Res<Time>, mut draw: Gizmos) {
63    if !config.fancy {
64        for _ in 0..(config.line_count / SYSTEM_COUNT) {
65            draw.line(Vec3::NEG_Y, Vec3::Y, Color::BLACK);
66        }
67    } else {
68        for i in 0..(config.line_count / SYSTEM_COUNT) {
69            let angle = i as f32 / (config.line_count / SYSTEM_COUNT) as f32 * TAU;
70
71            let vector = Vec2::from(ops::sin_cos(angle)).extend(ops::sin(time.elapsed_secs()));
72            let start_color = LinearRgba::rgb(vector.x, vector.z, 0.5);
73            let end_color = LinearRgba::rgb(-vector.z, -vector.y, 0.5);
74
75            draw.line_gradient(vector, -vector, start_color, end_color);
76        }
77    }
78}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#456)

#### pub fn [ray](#method.ray)(&mut self, start: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), vector: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>)

Draw a line in 3D from `start` to `start + vector`.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.ray(Vec3::Y, Vec3::X, GREEN);
}
```

##### [Examples found in repository](#scraped-examples-19)[?](../../scrape-examples-help.html)

examples/gizmos/3d\_gizmos.rs ([lines 168-172](../../src/3d_gizmos/3d_gizmos.rs.html#168-172))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#476-482)

#### pub fn [ray\_gradient](#method.ray_gradient)<C>( &mut self, start: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), vector: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), start\_color: C, end\_color: C, )

where C: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>,

Draw a line in 3D with a color gradient from `start` to `start + vector`.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.ray_gradient(Vec3::Y, Vec3::X, GREEN, RED);
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#502-506)

#### pub fn [linestrip](#method.linestrip)( &mut self, positions: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>, color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a line in 3D made of straight segments between the points.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.linestrip([Vec3::ZERO, Vec3::X, Vec3::Y], GREEN);
}
```

##### [Examples found in repository](#scraped-examples-20)[?](../../scrape-examples-help.html)

examples/math/cubic\_splines.rs ([lines 175-178](../../src/cubic_splines/cubic_splines.rs.html#175-178))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#530)

#### pub fn [lineloop](#method.lineloop)( &mut self, positions: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>, color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a line in 3D made of straight segments between the points, with the first and last connected.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.lineloop([Vec3::ZERO, Vec3::X, Vec3::Y], GREEN);
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#569-572)

#### pub fn [linestrip\_gradient](#method.linestrip_gradient)<C>( &mut self, points: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), C)>, )

where C: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>,

Draw a line in 3D made of straight segments between the points, with a color gradient.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.linestrip_gradient([
        (Vec3::ZERO, GREEN),
        (Vec3::X, RED),
        (Vec3::Y, BLUE)
    ]);
}
```

##### [Examples found in repository](#scraped-examples-21)[?](../../scrape-examples-help.html)

examples/3d/mesh\_ray\_cast.rs ([line 67](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#67))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#615)

#### pub fn [rect](#method.rect)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, size: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a wireframe rectangle in 3D with the given `isometry` applied.

If `isometry == Isometry3d::IDENTITY` then

*   the center is at `Vec3::ZERO`
*   the sizes are aligned with the `Vec3::X` and `Vec3::Y` axes.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.rect(Isometry3d::IDENTITY, Vec2::ONE, GREEN);
}
```

##### [Examples found in repository](#scraped-examples-22)[?](../../scrape-examples-help.html)

examples/gizmos/3d\_gizmos.rs ([lines 138-145](../../src/3d_gizmos/3d_gizmos.rs.html#138-145))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#637)

#### pub fn [cube](#method.cube)(&mut self, transform: impl [TransformPoint](../prelude/trait.TransformPoint.html "trait bevy::prelude::TransformPoint"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>)

Draw a wireframe cube in 3D.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.cube(Transform::IDENTITY, GREEN);
}
```

##### [Examples found in repository](#scraped-examples-23)[?](../../scrape-examples-help.html)

examples/3d/irradiance\_volumes.rs ([line 607](../../src/irradiance_volumes/irradiance_volumes.rs.html#607))

```rust
600fn draw_gizmo(
601    mut gizmos: Gizmos,
602    irradiance_volume_query: Query<&GlobalTransform, With<IrradianceVolume>>,
603    app_status: Res<AppStatus>,
604) {
605    if app_status.voxels_visible {
606        for transform in irradiance_volume_query.iter() {
607            gizmos.cube(*transform, GIZMO_COLOR);
608        }
609    }
610}
```

Hide additional examples

examples/3d/light\_probe\_blending.rs ([line 623](../../src/light_probe_blending/light_probe_blending.rs.html#623))

```rust
611fn draw_gizmos(
612    light_probes: Query<(&LightProbe, &ParallaxCorrection, &Transform)>,
613    app_status: Res<AppStatus>,
614    mut gizmos: Gizmos,
615) {
616    // If the user has gizmos disabled, bail.
617    if matches!(app_status.gizmos_enabled, GizmosEnabled::Off) {
618        return;
619    }
620
621    for (light_probe, parallax_correction, transform) in &light_probes {
622        // Draw light probe bounds.
623        gizmos.cube(*transform, TAN);
624
625        // Draw light probe falloff.
626        gizmos.cube(
627            Transform {
628                scale: transform.scale * (Vec3::ONE - light_probe.falloff),
629                ..*transform
630            },
631            CRIMSON,
632        );
633
634        // Draw light probe parallax correction bounds.
635        if let ParallaxCorrection::Custom(parallax_correction_bounds) = *parallax_correction {
636            gizmos.cube(
637                Transform {
638                    scale: transform.scale * parallax_correction_bounds,
639                    ..*transform
640                },
641                CORNFLOWER_BLUE,
642            );
643        }
644    }
645}
```

examples/testbed/3d.rs ([lines 380-383](../../src/testbed_3d/3d.rs.html#380-383))

```rust
379    pub fn draw_gizmos(mut gizmos: Gizmos) {
380        gizmos.cube(
381            Transform::from_translation(Vec3::X * -1.75).with_scale(Vec3::splat(1.25)),
382            RED,
383        );
384        gizmos
385            .sphere(Isometry3d::from_translation(Vec3::X * -3.5), 0.75, GREEN)
386            .resolution(30_000 / 3);
387
388        gizmos.text(
389            Isometry3d::from_translation(Vec3::Y * 1.5),
390            "text gizmo",
391            0.3,
392            Vec2 { x: 0., y: 0. },
393            Color::WHITE,
394        );
395
396        // 3d grids with all variations of outer edges on or off
397        for i in 0..8 {
398            let x = 1.5 * (i % 4) as f32;
399            let y = 1.0 * (0.5 - (i / 4) as f32);
400            let mut grid = gizmos.grid_3d(
401                Isometry3d::from_translation(Vec3::new(x, y, 0.0)),
402                UVec3::new(5, 4, 3),
403                Vec3::splat(0.175),
404                Color::WHITE,
405            );
406            if i & 1 > 0 {
407                grid = grid.outer_edges_x();
408            }
409            if i & 2 > 0 {
410                grid = grid.outer_edges_y();
411            }
412            if i & 4 > 0 {
413                grid.outer_edges_z();
414            }
415        }
416    }
```

examples/gizmos/3d\_gizmos.rs ([lines 134-137](../../src/3d_gizmos/3d_gizmos.rs.html#134-137))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#676-681)

#### pub fn [aabb\_3d](#method.aabb_3d)( &mut self, aabb: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Aabb3d](../math/bounding/struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")\>, transform: impl [TransformPoint](../prelude/trait.TransformPoint.html "trait bevy::prelude::TransformPoint"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a wireframe aabb in 3D.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.aabb_3d(Aabb3d::new(Vec3::ZERO, Vec3::ONE), Transform::IDENTITY, GREEN);
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#726)

#### pub fn [line\_2d](#method.line_2d)(&mut self, start: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), end: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>)

Draw a line in 2D from `start` to `end`.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.line_2d(Vec2::ZERO, Vec2::X, GREEN);
}
```

##### [Examples found in repository](#scraped-examples-24)[?](../../scrape-examples-help.html)

examples/math/bounding\_2d.rs ([lines 277-281](../../src/bounding_2d/bounding_2d.rs.html#277-281))

```rust
276fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
277    gizmos.line_2d(
278        ray.ray.origin,
279        ray.ray.origin + *ray.ray.direction * ray.max,
280        WHITE,
281    );
282    draw_filled_circle(gizmos, ray.ray.origin, FUCHSIA);
283}
```

Hide additional examples

examples/gizmos/2d\_gizmos.rs ([line 47](../../src/2d_gizmos/2d_gizmos.rs.html#47))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#746-752)

#### pub fn [line\_gradient\_2d](#method.line_gradient_2d)<C>( &mut self, start: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), end: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), start\_color: C, end\_color: C, )

where C: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>,

Draw a line in 2D with a color gradient from `start` to `end`.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.line_gradient_2d(Vec2::ZERO, Vec2::X, GREEN, RED);
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#772-776)

#### pub fn [linestrip\_2d](#method.linestrip_2d)( &mut self, positions: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>, color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a line in 2D made of straight segments between the points.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.linestrip_2d([Vec2::ZERO, Vec2::X, Vec2::Y], GREEN);
}
```

##### [Examples found in repository](#scraped-examples-25)[?](../../scrape-examples-help.html)

examples/animation/easing\_functions.rs ([lines 145-154](../../src/easing_functions/easing_functions.rs.html#145-154))

```rust
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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#796-800)

#### pub fn [lineloop\_2d](#method.lineloop_2d)( &mut self, positions: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>, color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a line in 2D made of straight segments between the points, with the first and last connected.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.lineloop_2d([Vec2::ZERO, Vec2::X, Vec2::Y], GREEN);
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#824-827)

#### pub fn [linestrip\_gradient\_2d](#method.linestrip_gradient_2d)<C>( &mut self, positions: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), C)>, )

where C: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>,

Draw a line in 2D made of straight segments between the points, with a color gradient.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.linestrip_gradient_2d([
        (Vec2::ZERO, GREEN),
        (Vec2::X, RED),
        (Vec2::Y, BLUE)
    ]);
}
```

##### [Examples found in repository](#scraped-examples-26)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([lines 61-66](../../src/2d_gizmos/2d_gizmos.rs.html#61-66))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#851)

#### pub fn [ray\_2d](#method.ray_2d)(&mut self, start: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), vector: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>)

Draw a line in 2D from `start` to `start + vector`.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.ray_2d(Vec2::Y, Vec2::X, GREEN);
}
```

##### [Examples found in repository](#scraped-examples-27)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([line 48](../../src/2d_gizmos/2d_gizmos.rs.html#48))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#871-877)

#### pub fn [ray\_gradient\_2d](#method.ray_gradient_2d)<C>( &mut self, start: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), vector: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), start\_color: C, end\_color: C, )

where C: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>,

Draw a line in 2D with a color gradient from `start` to `start + vector`.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.line_gradient(Vec3::Y, Vec3::X, GREEN, RED);
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#902-907)

#### pub fn [rect\_2d](#method.rect_2d)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>, size: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw a wireframe rectangle in 2D with the given `isometry` applied.

If `isometry == Isometry2d::IDENTITY` then

*   the center is at `Vec2::ZERO`
*   the sizes are aligned with the `Vec2::X` and `Vec2::Y` axes.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.rect_2d(Isometry2d::IDENTITY, Vec2::ONE, GREEN);
}
```

##### [Examples found in repository](#scraped-examples-28)[?](../../scrape-examples-help.html)

examples/math/bounding\_2d.rs ([line 187](../../src/bounding_2d/bounding_2d.rs.html#187))

```rust
182fn render_volumes(mut gizmos: Gizmos, query: Query<(&CurrentVolume, &Intersects)>) {
183    for (volume, intersects) in query.iter() {
184        let color = if **intersects { AQUA } else { ORANGE_RED };
185        match volume {
186            CurrentVolume::Aabb(a) => {
187                gizmos.rect_2d(a.center(), a.half_size() * 2., color);
188            }
189            CurrentVolume::Circle(c) => {
190                gizmos.circle_2d(c.center(), c.radius(), color);
191            }
192        }
193    }
194}
195
196#[derive(Component, Deref, DerefMut, Default)]
197struct Intersects(bool);
198
199const OFFSET_X: f32 = 125.;
200const OFFSET_Y: f32 = 75.;
201
202fn setup(mut commands: Commands) {
203    commands.spawn(Camera2d);
204
205    commands.spawn((
206        Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.),
207        Shape::Circle(Circle::new(45.)),
208        DesiredVolume::Aabb,
209        Intersects::default(),
210    ));
211
212    commands.spawn((
213        Transform::from_xyz(0., OFFSET_Y, 0.),
214        Shape::Rectangle(Rectangle::new(80., 80.)),
215        Spin,
216        DesiredVolume::Circle,
217        Intersects::default(),
218    ));
219
220    commands.spawn((
221        Transform::from_xyz(OFFSET_X, OFFSET_Y, 0.),
222        Shape::Triangle(Triangle2d::new(
223            Vec2::new(-40., -40.),
224            Vec2::new(-20., 40.),
225            Vec2::new(40., 50.),
226        )),
227        Spin,
228        DesiredVolume::Aabb,
229        Intersects::default(),
230    ));
231
232    commands.spawn((
233        Transform::from_xyz(-OFFSET_X, -OFFSET_Y, 0.),
234        Shape::Line(Segment2d::from_direction_and_length(
235            Dir2::from_xy(1., 0.3).unwrap(),
236            90.,
237        )),
238        Spin,
239        DesiredVolume::Circle,
240        Intersects::default(),
241    ));
242
243    commands.spawn((
244        Transform::from_xyz(0., -OFFSET_Y, 0.),
245        Shape::Capsule(Capsule2d::new(25., 50.)),
246        Spin,
247        DesiredVolume::Aabb,
248        Intersects::default(),
249    ));
250
251    commands.spawn((
252        Transform::from_xyz(OFFSET_X, -OFFSET_Y, 0.),
253        Shape::Polygon(RegularPolygon::new(50., 6)),
254        Spin,
255        DesiredVolume::Circle,
256        Intersects::default(),
257    ));
258
259    commands.spawn((
260        Text::default(),
261        Node {
262            position_type: PositionType::Absolute,
263            top: px(12),
264            left: px(12),
265            ..default()
266        },
267    ));
268}
269
270fn draw_filled_circle(gizmos: &mut Gizmos, position: Vec2, color: Srgba) {
271    for r in [1., 2., 3.] {
272        gizmos.circle_2d(position, r, color);
273    }
274}
275
276fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
277    gizmos.line_2d(
278        ray.ray.origin,
279        ray.ray.origin + *ray.ray.direction * ray.max,
280        WHITE,
281    );
282    draw_filled_circle(gizmos, ray.ray.origin, FUCHSIA);
283}
284
285fn get_and_draw_ray(gizmos: &mut Gizmos, time: &Time) -> RayCast2d {
286    let ray = Vec2::new(ops::cos(time.elapsed_secs()), ops::sin(time.elapsed_secs()));
287    let dist = 150. + ops::sin(0.5 * time.elapsed_secs()).abs() * 500.;
288
289    let aabb_ray = Ray2d {
290        origin: ray * 250.,
291        direction: Dir2::new_unchecked(-ray),
292    };
293    let ray_cast = RayCast2d::from_ray(aabb_ray, dist - 20.);
294
295    draw_ray(gizmos, &ray_cast);
296    ray_cast
297}
298
299fn ray_cast_system(
300    mut gizmos: Gizmos,
301    time: Res<Time>,
302    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
303) {
304    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
305
306    for (volume, mut intersects) in volumes.iter_mut() {
307        let toi = match volume {
308            CurrentVolume::Aabb(a) => ray_cast.aabb_intersection_at(a),
309            CurrentVolume::Circle(c) => ray_cast.circle_intersection_at(c),
310        };
311        **intersects = toi.is_some();
312        if let Some(toi) = toi {
313            draw_filled_circle(
314                &mut gizmos,
315                ray_cast.ray.origin + *ray_cast.ray.direction * toi,
316                LIME,
317            );
318        }
319    }
320}
321
322fn aabb_cast_system(
323    mut gizmos: Gizmos,
324    time: Res<Time>,
325    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
326) {
327    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
328    let aabb_cast = AabbCast2d {
329        aabb: Aabb2d::new(Vec2::ZERO, Vec2::splat(15.)),
330        ray: ray_cast,
331    };
332
333    for (volume, mut intersects) in volumes.iter_mut() {
334        let toi = match *volume {
335            CurrentVolume::Aabb(a) => aabb_cast.aabb_collision_at(a),
336            CurrentVolume::Circle(_) => None,
337        };
338
339        **intersects = toi.is_some();
340        if let Some(toi) = toi {
341            gizmos.rect_2d(
342                aabb_cast.ray.ray.origin + *aabb_cast.ray.ray.direction * toi,
343                aabb_cast.aabb.half_size() * 2.,
344                LIME,
345            );
346        }
347    }
348}
349
350fn bounding_circle_cast_system(
351    mut gizmos: Gizmos,
352    time: Res<Time>,
353    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
354) {
355    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
356    let circle_cast = BoundingCircleCast {
357        circle: BoundingCircle::new(Vec2::ZERO, 15.),
358        ray: ray_cast,
359    };
360
361    for (volume, mut intersects) in volumes.iter_mut() {
362        let toi = match *volume {
363            CurrentVolume::Aabb(_) => None,
364            CurrentVolume::Circle(c) => circle_cast.circle_collision_at(c),
365        };
366
367        **intersects = toi.is_some();
368        if let Some(toi) = toi {
369            gizmos.circle_2d(
370                circle_cast.ray.ray.origin + *circle_cast.ray.ray.direction * toi,
371                circle_cast.circle.radius(),
372                LIME,
373            );
374        }
375    }
376}
377
378fn get_intersection_position(time: &Time) -> Vec2 {
379    let x = ops::cos(0.8 * time.elapsed_secs()) * 250.;
380    let y = ops::sin(0.4 * time.elapsed_secs()) * 100.;
381    Vec2::new(x, y)
382}
383
384fn aabb_intersection_system(
385    mut gizmos: Gizmos,
386    time: Res<Time>,
387    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
388) {
389    let center = get_intersection_position(&time);
390    let aabb = Aabb2d::new(center, Vec2::splat(50.));
391    gizmos.rect_2d(center, aabb.half_size() * 2., YELLOW);
392
393    for (volume, mut intersects) in volumes.iter_mut() {
394        let hit = match volume {
395            CurrentVolume::Aabb(a) => aabb.intersects(a),
396            CurrentVolume::Circle(c) => aabb.intersects(c),
397        };
398
399        **intersects = hit;
400    }
401}
```

Hide additional examples

examples/2d/mesh2d\_arcs.rs ([line 115](../../src/mesh2d_arcs/mesh2d_arcs.rs.html#115))

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

examples/testbed/2d.rs ([lines 395-399](../../src/testbed_2d/2d.rs.html#395-399))

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

examples/math/custom\_primitives.rs ([line 299](../../src/custom_primitives/custom_primitives.rs.html#299))

```rust
283fn bounding_shapes_2d(
284    shapes: Query<&Transform, With<Shape2d>>,
285    mut gizmos: Gizmos,
286    bounding_shape: Res<State<BoundingShape>>,
287) {
288    for transform in shapes.iter() {
289        // Get the rotation angle from the 3D rotation.
290        let rotation = transform.rotation.to_scaled_axis().z;
291        let rotation = Rot2::radians(rotation);
292        let isometry = Isometry2d::new(transform.translation.xy(), rotation);
293
294        match bounding_shape.get() {
295            BoundingShape::None => (),
296            BoundingShape::BoundingBox => {
297                // Get the AABB of the primitive with the rotation and translation of the mesh.
298                let aabb = HEART.aabb_2d(isometry);
299                gizmos.rect_2d(aabb.center(), aabb.half_size() * 2., WHITE);
300            }
301            BoundingShape::BoundingSphere => {
302                // Get the bounding sphere of the primitive with the rotation and translation of the mesh.
303                let bounding_circle = HEART.bounding_circle(isometry);
304                gizmos
305                    .circle_2d(bounding_circle.center(), bounding_circle.radius(), WHITE)
306                    .resolution(64);
307            }
308        }
309    }
310}
```

examples/gizmos/2d\_gizmos.rs ([line 68](../../src/2d_gizmos/2d_gizmos.rs.html#68))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/grid.rs.html#217-223)

#### pub fn [grid](#method.grid)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, cell\_count: [UVec2](../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), spacing: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [GridBuilder2d](grid/struct.GridBuilder2d.html "struct bevy::gizmos::grid::GridBuilder2d")<'\_, Config, Clear>

Draw a 2D grid in 3D.

The grid’s default orientation aligns with the XY-plane.

##### Arguments

*   `isometry` defines the translation and rotation of the grid.
    *   the translation specifies the center of the grid
    *   defines the orientation of the grid, by default we assume the grid is contained in a plane parallel to the XY plane
*   `cell_count`: defines the amount of cells in the x and y axes
*   `spacing`: defines the distance between cells along the x and y axes
*   `color`: color of the grid

##### Builder methods

*   The skew of the grid can be adjusted using the `.skew(...)`, `.skew_x(...)` or `.skew_y(...)` methods. They behave very similar to their CSS equivalents.
*   All outer edges can be toggled on or off using `.outer_edges(...)`. Alternatively you can use `.outer_edges_x(...)` or `.outer_edges_y(...)` to toggle the outer edges along an axis.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.grid(
        Isometry3d::IDENTITY,
        UVec2::new(10, 10),
        Vec2::splat(2.),
        GREEN
        )
        .skew_x(0.25)
        .outer_edges();
}
```

##### [Examples found in repository](#scraped-examples-29)[?](../../scrape-examples-help.html)

examples/testbed/2d.rs ([lines 420-425](../../src/testbed_2d/2d.rs.html#420-425))

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

Hide additional examples

examples/gizmos/3d\_gizmos.rs ([lines 104-110](../../src/3d_gizmos/3d_gizmos.rs.html#104-110))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/grid.rs.html#268-274)

#### pub fn [grid\_3d](#method.grid_3d)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, cell\_count: [UVec3](../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"), spacing: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [GridBuilder3d](grid/struct.GridBuilder3d.html "struct bevy::gizmos::grid::GridBuilder3d")<'\_, Config, Clear>

Draw a 3D grid of voxel-like cells.

##### Arguments

*   `isometry` defines the translation and rotation of the grid.
    *   the translation specifies the center of the grid
    *   defines the orientation of the grid, by default we assume the grid is aligned with all axes
*   `cell_count`: defines the amount of cells in the x, y and z axes
*   `spacing`: defines the distance between cells along the x, y and z axes
*   `color`: color of the grid

##### Builder methods

*   The skew of the grid can be adjusted using the `.skew(...)`, `.skew_x(...)`, `.skew_y(...)` or `.skew_z(...)` methods. They behave very similar to their CSS equivalents.
*   All outer edges can be toggled on or off using `.outer_edges(...)`. Alternatively you can use `.outer_edges_x(...)`, `.outer_edges_y(...)` or `.outer_edges_z(...)` to toggle the outer edges along an axis.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.grid_3d(
        Isometry3d::IDENTITY,
        UVec3::new(10, 2, 10),
        Vec3::splat(2.),
        GREEN
        )
        .skew_x(0.25)
        .outer_edges();
}
```

##### [Examples found in repository](#scraped-examples-30)[?](../../scrape-examples-help.html)

examples/testbed/3d.rs ([lines 400-405](../../src/testbed_3d/3d.rs.html#400-405))

```rust
379    pub fn draw_gizmos(mut gizmos: Gizmos) {
380        gizmos.cube(
381            Transform::from_translation(Vec3::X * -1.75).with_scale(Vec3::splat(1.25)),
382            RED,
383        );
384        gizmos
385            .sphere(Isometry3d::from_translation(Vec3::X * -3.5), 0.75, GREEN)
386            .resolution(30_000 / 3);
387
388        gizmos.text(
389            Isometry3d::from_translation(Vec3::Y * 1.5),
390            "text gizmo",
391            0.3,
392            Vec2 { x: 0., y: 0. },
393            Color::WHITE,
394        );
395
396        // 3d grids with all variations of outer edges on or off
397        for i in 0..8 {
398            let x = 1.5 * (i % 4) as f32;
399            let y = 1.0 * (0.5 - (i / 4) as f32);
400            let mut grid = gizmos.grid_3d(
401                Isometry3d::from_translation(Vec3::new(x, y, 0.0)),
402                UVec3::new(5, 4, 3),
403                Vec3::splat(0.175),
404                Color::WHITE,
405            );
406            if i & 1 > 0 {
407                grid = grid.outer_edges_x();
408            }
409            if i & 2 > 0 {
410                grid = grid.outer_edges_y();
411            }
412            if i & 4 > 0 {
413                grid.outer_edges_z();
414            }
415        }
416    }
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/grid.rs.html#319-325)

#### pub fn [grid\_2d](#method.grid_2d)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>, cell\_count: [UVec2](../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"), spacing: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [GridBuilder2d](grid/struct.GridBuilder2d.html "struct bevy::gizmos::grid::GridBuilder2d")<'\_, Config, Clear>

Draw a grid in 2D.

##### Arguments

*   `isometry` defines the translation and rotation of the grid.
    *   the translation specifies the center of the grid
    *   defines the orientation of the grid, by default we assume the grid is aligned with all axes
*   `cell_count`: defines the amount of cells in the x and y axes
*   `spacing`: defines the distance between cells along the x and y axes
*   `color`: color of the grid

##### Builder methods

*   The skew of the grid can be adjusted using the `.skew(...)`, `.skew_x(...)` or `.skew_y(...)` methods. They behave very similar to their CSS equivalents.
*   All outer edges can be toggled on or off using `.outer_edges(...)`. Alternatively you can use `.outer_edges_x(...)` or `.outer_edges_y(...)` to toggle the outer edges along an axis.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(10, 10),
        Vec2::splat(1.),
        GREEN
        )
        .skew_x(0.25)
        .outer_edges();
}
```

##### [Examples found in repository](#scraped-examples-31)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([lines 51-57](../../src/2d_gizmos/2d_gizmos.rs.html#51-57))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/rounded_box.rs.html#267-272)

#### pub fn [rounded\_rect](#method.rounded_rect)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, size: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [RoundedRectBuilder](rounded_box/struct.RoundedRectBuilder.html "struct bevy::gizmos::rounded_box::RoundedRectBuilder")<'\_, Config, Clear>

Draw a wireframe rectangle with rounded corners in 3D.

##### Arguments

*   `isometry` defines the translation and rotation of the rectangle.
    *   the translation specifies the center of the rectangle
    *   defines orientation of the rectangle, by default we assume the rectangle is contained in a plane parallel to the XY plane.
*   `size`: defines the size of the rectangle. This refers to the ‘outer size’, similar to a bounding box.
*   `color`: color of the rectangle

##### Builder methods

*   The corner radius can be adjusted with the `.corner_radius(...)` method.
*   The resolution of the arcs at each corner (i.e. the level of detail) can be adjusted with the `.arc_resolution(...)` method.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.rounded_rect(
        Isometry3d::IDENTITY,
        Vec2::ONE,
        GREEN
        )
        .corner_radius(0.25)
        .arc_resolution(10);
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/rounded_box.rs.html#318-323)

#### pub fn [rounded\_rect\_2d](#method.rounded_rect_2d)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>, size: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [RoundedRectBuilder](rounded_box/struct.RoundedRectBuilder.html "struct bevy::gizmos::rounded_box::RoundedRectBuilder")<'\_, Config, Clear>

Draw a wireframe rectangle with rounded corners in 2D.

##### Arguments

*   `isometry` defines the translation and rotation of the rectangle.
    *   the translation specifies the center of the rectangle
    *   defines orientation of the rectangle, by default we assume the rectangle aligned with all axes.
*   `size`: defines the size of the rectangle. This refers to the ‘outer size’, similar to a bounding box.
*   `color`: color of the rectangle

##### Builder methods

*   The corner radius can be adjusted with the `.corner_radius(...)` method.
*   The resolution of the arcs at each corner (i.e. the level of detail) can be adjusted with the `.arc_resolution(...)` method.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.rounded_rect_2d(
        Isometry2d::IDENTITY,
        Vec2::ONE,
        GREEN
        )
        .corner_radius(0.25)
        .arc_resolution(10);
}
```

##### [Examples found in repository](#scraped-examples-32)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([line 82](../../src/2d_gizmos/2d_gizmos.rs.html#82))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/rounded_box.rs.html#373-378)

#### pub fn [rounded\_cuboid](#method.rounded_cuboid)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, size: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [RoundedCuboidBuilder](rounded_box/struct.RoundedCuboidBuilder.html "struct bevy::gizmos::rounded_box::RoundedCuboidBuilder")<'\_, Config, Clear>

Draw a wireframe cuboid with rounded corners in 3D.

##### Arguments

*   `isometry` defines the translation and rotation of the cuboid.
    *   the translation specifies the center of the cuboid
    *   defines orientation of the cuboid, by default we assume the cuboid aligned with all axes.
*   `size`: defines the size of the cuboid. This refers to the ‘outer size’, similar to a bounding box.
*   `color`: color of the cuboid

##### Builder methods

*   The edge radius can be adjusted with the `.edge_radius(...)` method.
*   The resolution of the arcs at each edge (i.e. the level of detail) can be adjusted with the `.arc_resolution(...)` method.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.rounded_cuboid(
        Isometry3d::IDENTITY,
        Vec3::ONE,
        GREEN
        )
        .edge_radius(0.25)
        .arc_resolution(10);
}
```

##### [Examples found in repository](#scraped-examples-33)[?](../../scrape-examples-help.html)

examples/gizmos/3d\_gizmos.rs ([line 163](../../src/3d_gizmos/3d_gizmos.rs.html#163))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/stroke_text.rs.html#197-204)

#### pub fn [text](#method.text)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, text: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), font\_size: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), anchor: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw text using a stroke font with the given isometry applied.

Only ASCII characters in the range 32–126 are supported.

##### Arguments

*   `isometry`: defines the translation and rotation of the text.
*   `text`: the text to be drawn.
*   `size`: the size of the text in pixels.
*   `anchor`: normalized anchor point relative to the text bounds, where `(0, 0)` is centered, `(-0.5, 0.5)` is top-left, and `(0.5, -0.5)` is bottom-right.
*   `color`: the color of the text.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.text(Isometry3d::IDENTITY, "text gizmo", 25., Vec2::ZERO, Color::WHITE);
}
```

##### [Examples found in repository](#scraped-examples-34)[?](../../scrape-examples-help.html)

examples/gizmos/3d\_text\_gizmos.rs ([lines 28-34](../../src/3d_text_gizmos/3d_text_gizmos.rs.html#28-34))

```rust
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

Hide additional examples

examples/testbed/3d.rs ([lines 388-394](../../src/testbed_3d/3d.rs.html#388-394))

```rust
379    pub fn draw_gizmos(mut gizmos: Gizmos) {
380        gizmos.cube(
381            Transform::from_translation(Vec3::X * -1.75).with_scale(Vec3::splat(1.25)),
382            RED,
383        );
384        gizmos
385            .sphere(Isometry3d::from_translation(Vec3::X * -3.5), 0.75, GREEN)
386            .resolution(30_000 / 3);
387
388        gizmos.text(
389            Isometry3d::from_translation(Vec3::Y * 1.5),
390            "text gizmo",
391            0.3,
392            Vec2 { x: 0., y: 0. },
393            Color::WHITE,
394        );
395
396        // 3d grids with all variations of outer edges on or off
397        for i in 0..8 {
398            let x = 1.5 * (i % 4) as f32;
399            let y = 1.0 * (0.5 - (i / 4) as f32);
400            let mut grid = gizmos.grid_3d(
401                Isometry3d::from_translation(Vec3::new(x, y, 0.0)),
402                UVec3::new(5, 4, 3),
403                Vec3::splat(0.175),
404                Color::WHITE,
405            );
406            if i & 1 > 0 {
407                grid = grid.outer_edges_x();
408            }
409            if i & 2 > 0 {
410                grid = grid.outer_edges_y();
411            }
412            if i & 4 > 0 {
413                grid.outer_edges_z();
414            }
415        }
416    }
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/stroke_text.rs.html#239-245)

#### pub fn [text\_sections](#method.text_sections)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, sections: &\[(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), [Color](../prelude/enum.Color.html "enum bevy::prelude::Color"))\], font\_size: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), anchor: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), )

Draw text using a stroke font with the given isometry applied, coloring each section independently.

Only ASCII characters in the range 32–126 are supported.

##### Arguments

*   `isometry`: defines the translation and rotation of the text.
*   `sections`: a slice of `(text, color)` pairs. Each section’s characters are drawn in its color. Sections are concatenated left-to-right on the same baseline.
*   `font_size`: the size of the text in pixels.
*   `anchor`: normalized anchor point relative to the combined text bounds, where `(0, 0)` is centered, `(-0.5, 0.5)` is top-left, and `(0.5, -0.5)` is bottom-right.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.text_sections(
        Isometry3d::IDENTITY,
        &[("Hello ", Color::WHITE), ("World!", Color::srgb(1., 0.3, 0.))],
        25.,
        Vec2::ZERO,
    );
}
```

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/stroke_text.rs.html#281-288)

#### pub fn [text\_2d](#method.text_2d)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>, text: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), font\_size: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), anchor: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../prelude/enum.Color.html "enum bevy::prelude::Color")\>, )

Draw text using a stroke font in 2d with the given isometry applied.

Only ASCII characters in the range 32–126 are supported.

##### Arguments

*   `isometry`: defines the translation and rotation of the text.
*   `text`: the text to be drawn.
*   `size`: the size of the text.
*   `anchor`: normalized anchor point relative to the text bounds, where `(0., 0.)` is centered, `(-0.5, 0.5)` is top-left, and `(0.5, -0.5)` is bottom-right.
*   `color`: the color of the text.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.text_2d(Isometry2d::IDENTITY, "2D text gizmo", 25., Vec2::ZERO, Color::WHITE);
}
```

##### [Examples found in repository](#scraped-examples-35)[?](../../scrape-examples-help.html)

examples/gizmos/text\_gizmos\_font.rs ([lines 29-35](../../src/text_gizmos_font/text_gizmos_font.rs.html#29-35))

```rust
28fn draw_all_glyphs(mut text_gizmos: Gizmos) {
29    text_gizmos.text_2d(
30        Isometry2d::IDENTITY,
31        ALL_GLYPHS,
32        40.0,
33        Vec2::ZERO,
34        Color::WHITE,
35    );
36}
```

Hide additional examples

examples/gizmos/anchored\_text\_gizmos.rs ([lines 31-37](../../src/anchored_text_gizmos/anchored_text_gizmos.rs.html#31-37))

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

examples/gizmos/2d\_text\_gizmos.rs ([lines 50-62](../../src/2d_text_gizmos/2d_text_gizmos.rs.html#50-62))

```rust
44fn draw_labels(mut text_gizmos: Gizmos, diagnostic: Res<DiagnosticsStore>) {
45    let colors = [RED, GREEN, BLUE, YELLOW];
46    for i in 0..TEXT_COUNT {
47        let row = i / 5;
48        let col = i % 5;
49        let color = colors[i % 4];
50        text_gizmos.text_2d(
51            Isometry2d {
52                translation: Vec2::new(
53                    START_X + col as f32 * X_STEP,
54                    START_Y - row as f32 * Y_STEP,
55                ),
56                rotation: Rot2::degrees(2.),
57            },
58            &format!("label {i}"),
59            25.,
60            Vec2::ZERO,
61            color,
62        );
63    }
64
65    if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS)
66        && let Some(fps_smoothed) = fps.smoothed()
67    {
68        text_gizmos.text_2d(
69            Isometry2d::from_translation(Vec2::new(600., START_Y + 150.)),
70            &format!("fps: {:.1}", fps_smoothed),
71            25.,
72            Vec2::ZERO,
73            Color::WHITE,
74        );
75    }
76
77    text_gizmos.text_2d(
78        Isometry2d::from_translation(Vec2::new(-300., START_Y + 200.)),
79        "lxgh",
80        150.,
81        Vec2::ZERO,
82        Color::WHITE,
83    );
84}
85
86const ALL_GLYPHS: &str = " !\"#$%&'()*\n\
87+,-./012345\n\
886789:;<=>?@\n\
89ABCDEFGHIJK\n\
90LMNOPQRSTUV\n\
91WXYZ[\\]^_`a\n\
92bcdefghijkl\n\
93mnopqrstuvw\n\
94xyz{|}~";
95
96fn draw_all_glyphs(mut text_gizmos: Gizmos) {
97    text_gizmos.text_2d(
98        Isometry2d::from_xy(600., 0.),
99        ALL_GLYPHS,
100        30.0,
101        Vec2::ZERO,
102        Color::WHITE,
103    );
104}
```

examples/testbed/2d.rs ([lines 408-414](../../src/testbed_2d/2d.rs.html#408-414))

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

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/stroke_text.rs.html#323-329)

#### pub fn [text\_sections\_2d](#method.text_sections_2d)( &mut self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>, sections: &\[(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), [Color](../prelude/enum.Color.html "enum bevy::prelude::Color"))\], font\_size: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), anchor: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), )

Draw text using a stroke font in 2d with the given isometry applied, coloring each section independently.

Only ASCII characters in the range 32–126 are supported.

##### Arguments

*   `isometry`: defines the translation and rotation of the text.
*   `sections`: a slice of `(text, color)` pairs. Each section’s characters are drawn in its color. Sections are concatenated left-to-right on the same baseline.
*   `font_size`: the size of the text.
*   `anchor`: normalized anchor point relative to the combined text bounds, where `(0., 0.)` is centered, `(-0.5, 0.5)` is top-left, and `(0.5, -0.5)` is bottom-right.

##### Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.text_sections_2d(
        Isometry2d::IDENTITY,
        &[("Hello ", Color::WHITE), ("World!", Color::srgb(1., 0.3, 0.))],
        25.,
        Vec2::ZERO,
    );
}
```

## Trait Implementations

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

### impl [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#354)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#355)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#16)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#17)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [GizmoBuffer](gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<[ErasedGizmoConfigGroup](config/struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup"), [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#19)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#24)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#25)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

### impl [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

### impl [VisitAssetDependencies](../asset/trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

#### fn [visit\_dependencies](../asset/trait.VisitAssetDependencies.html#tymethod.visit_dependencies)(&self, visit: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([UntypedAssetId](../asset/enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")))

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [GizmoAsset](../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

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

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](../prelude/trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](../prelude/trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](../prelude/trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

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

### impl<T> [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

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

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

### impl<T> [Template](../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#method.clone_into)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}