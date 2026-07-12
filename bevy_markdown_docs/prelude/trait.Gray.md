[bevy](../index.html)::[prelude](index.html)

# Trait Gray 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#46)

```rust
pub trait Gray: Sized + Mix {
    const BLACK: Self;
    const WHITE: Self;

    // Provided method
    fn gray(lightness: f32) -> Self { ... }
}
```

Trait for returning a grayscale color of a provided lightness.

## Required Associated Constants

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#48)

#### const [BLACK](#associatedconstant.BLACK): Self

A pure black color.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#50)

#### const [WHITE](#associatedconstant.WHITE): Self

A pure white color.

## Provided Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#53)

#### fn [gray](#method.gray)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Returns a grey color with the provided lightness from (0.0 - 1.0). 0 is black, 1 is white.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/gizmos/2d\_gizmos.rs ([line 56](../../src/2d_gizmos/2d_gizmos.rs.html#56))

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

Hide additional examples

examples/gizmos/3d\_gizmos.rs ([line 109](../../src/3d_gizmos/3d_gizmos.rs.html#109))

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

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#127)

### impl [Gray](trait.Gray.html "trait bevy::prelude::Gray") for [Hsla](struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#128)

#### const [BLACK](#associatedconstant.BLACK): [Hsla](struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#129)

#### const [WHITE](#associatedconstant.WHITE): [Hsla](struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#98)

### impl [Gray](trait.Gray.html "trait bevy::prelude::Gray") for [Hsva](struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#99)

#### const [BLACK](#associatedconstant.BLACK): [Hsva](struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#100)

#### const [WHITE](#associatedconstant.WHITE): [Hsva](struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#101)

### impl [Gray](trait.Gray.html "trait bevy::prelude::Gray") for [Hwba](struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#102)

#### const [BLACK](#associatedconstant.BLACK): [Hwba](struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#103)

#### const [WHITE](#associatedconstant.WHITE): [Hwba](struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#109)

### impl [Gray](trait.Gray.html "trait bevy::prelude::Gray") for [Laba](struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#110)

#### const [BLACK](#associatedconstant.BLACK): [Laba](struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#111)

#### const [WHITE](#associatedconstant.WHITE): [Laba](struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#131)

### impl [Gray](trait.Gray.html "trait bevy::prelude::Gray") for [Lcha](struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#132)

#### const [BLACK](#associatedconstant.BLACK): [Lcha](struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#133)

#### const [WHITE](#associatedconstant.WHITE): [Lcha](struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#220)

### impl [Gray](trait.Gray.html "trait bevy::prelude::Gray") for [LinearRgba](struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#221)

#### const [BLACK](#associatedconstant.BLACK): [LinearRgba](struct.LinearRgba.html "struct bevy::prelude::LinearRgba") = Self::BLACK

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#222)

#### const [WHITE](#associatedconstant.WHITE): [LinearRgba](struct.LinearRgba.html "struct bevy::prelude::LinearRgba") = Self::WHITE

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#109)

### impl [Gray](trait.Gray.html "trait bevy::prelude::Gray") for [Oklaba](struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#110)

#### const [BLACK](#associatedconstant.BLACK): [Oklaba](struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#111)

#### const [WHITE](#associatedconstant.WHITE): [Oklaba](struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#126)

### impl [Gray](trait.Gray.html "trait bevy::prelude::Gray") for [Oklcha](struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#127)

#### const [BLACK](#associatedconstant.BLACK): [Oklcha](struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#128)

#### const [WHITE](#associatedconstant.WHITE): [Oklcha](struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#302)

### impl [Gray](trait.Gray.html "trait bevy::prelude::Gray") for [Srgba](struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#303)

#### const [BLACK](#associatedconstant.BLACK): [Srgba](struct.Srgba.html "struct bevy::prelude::Srgba") = Self::BLACK

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#304)

#### const [WHITE](#associatedconstant.WHITE): [Srgba](struct.Srgba.html "struct bevy::prelude::Srgba") = Self::WHITE

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#152)

### impl [Gray](trait.Gray.html "trait bevy::prelude::Gray") for [Xyza](struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#153)

#### const [BLACK](#associatedconstant.BLACK): [Xyza](struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#154)

#### const [WHITE](#associatedconstant.WHITE): [Xyza](struct.Xyza.html "struct bevy::prelude::Xyza")