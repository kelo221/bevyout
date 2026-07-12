[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function sin\_cos 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#380)

```rust
pub fn sin_cos(x: f32) -> (f32, f32)
```

Simultaneously computes the sine and cosine of the number, `x`. Returns `(sin(x), cos(x))`.

Precision is specified when the `libm` feature is enabled.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/app/render\_recovery.rs ([line 91](../../../src/render_recovery/render_recovery.rs.html#91))

```rust
89fn update_camera(mut camera: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
90    for mut t in camera.iter_mut() {
91        let (s, c) = ops::sin_cos(time.elapsed_secs() * 0.3);
92        *t = Transform::from_xyz(s * 10.0, 4.5, c * 10.0).looking_at(Vec3::ZERO, Vec3::Y);
93    }
94}
```

Hide additional examples

examples/app/externally\_driven\_headless\_renderer.rs ([line 156](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#156))

```rust
154fn update_camera(mut camera: Query<&mut Transform, With<Camera>>, frame_count: Res<FrameCount>) {
155    for mut t in camera.iter_mut() {
156        let (s, c) = ops::sin_cos(frame_count.0 as f32 * 0.3);
157        *t = Transform::from_xyz(s * 10.0, 4.5, c * 10.0).looking_at(Vec3::ZERO, Vec3::Y);
158    }
159}
```

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 432](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#432))

```rust
431    fn to_cartesian(self) -> Vec3 {
432        let (sin_inclination, cos_inclination) = sin_cos(self.inclination);
433        let (sin_azimuth, cos_azimuth) = sin_cos(self.azimuth);
434        self.radius
435            * vec3(
436                sin_inclination * cos_azimuth,
437                cos_inclination,
438                sin_inclination * sin_azimuth,
439            )
440    }
```

examples/stress\_tests/many\_gizmos.rs ([line 71](../../../src/many_gizmos/many_gizmos.rs.html#71))

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

examples/math/custom\_primitives.rs ([line 533](../../../src/custom_primitives/custom_primitives.rs.html#533))

```rust
514    fn build(&self) -> Mesh {
515        let radius = self.heart.radius;
516        // The curved parts of each wing (half) of the heart have an angle of `PI * 1.25` or 225°
517        let wing_angle = PI * 1.25;
518
519        // We create buffers for the vertices, their normals and UVs, as well as the indices used to connect the vertices.
520        let mut vertices = Vec::with_capacity(2 * self.resolution);
521        let mut uvs = Vec::with_capacity(2 * self.resolution);
522        let mut indices = Vec::with_capacity(6 * self.resolution - 9);
523        // Since the heart is flat, we know all the normals are identical already.
524        let normals = vec![[0f32, 0f32, 1f32]; 2 * self.resolution];
525
526        // The point in the middle of the two curved parts of the heart
527        vertices.push([0.0; 3]);
528        uvs.push([0.5, 0.5]);
529
530        // The left wing of the heart, starting from the point in the middle.
531        for i in 1..self.resolution {
532            let angle = (i as f32 / self.resolution as f32) * wing_angle;
533            let (sin, cos) = ops::sin_cos(angle);
534            vertices.push([radius * (cos - 1.0), radius * sin, 0.0]);
535            uvs.push([0.5 - (cos - 1.0) / 4., 0.5 - sin / 2.]);
536        }
537
538        // The bottom tip of the heart
539        vertices.push([0.0, radius * (-1. - SQRT_2), 0.0]);
540        uvs.push([0.5, 1.]);
541
542        // The right wing of the heart, starting from the bottom most point and going towards the middle point.
543        for i in 0..self.resolution - 1 {
544            let angle = (i as f32 / self.resolution as f32) * wing_angle - PI / 4.;
545            let (sin, cos) = ops::sin_cos(angle);
546            vertices.push([radius * (cos + 1.0), radius * sin, 0.0]);
547            uvs.push([0.5 - (cos + 1.0) / 4., 0.5 - sin / 2.]);
548        }
549
550        // This is where we build all the triangles from the points created above.
551        // Each triangle has one corner on the middle point with the other two being adjacent points on the perimeter of the heart.
552        for i in 2..2 * self.resolution as u32 {
553            indices.extend_from_slice(&[i - 1, i, 0]);
554        }
555
556        // Here, the actual `Mesh` is created. We set the indices, vertices, normals and UVs created above and specify the topology of the mesh.
557        Mesh::new(
558            bevy::mesh::PrimitiveTopology::TriangleList,
559            RenderAssetUsages::default(),
560        )
561        .with_inserted_indices(bevy::mesh::Indices::U32(indices))
562        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
563        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
564        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
565    }
```

examples/gizmos/3d\_gizmos.rs ([line 126](../../../src/3d_gizmos/3d_gizmos.rs.html#126))

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

Additional examples can be found in:  

*   [examples/stress\_tests/many\_cubes.rs](../../../src/many_cubes/many_cubes.rs.html#458)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#176)
*   [examples/usage/debug\_frustum\_culling.rs](../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#240)