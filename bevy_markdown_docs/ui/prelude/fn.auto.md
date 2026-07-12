[bevy](../../index.html)::[ui](../index.html)::[prelude](index.html)

# Function auto 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#536)

```rust
pub const fn auto() -> Val
```

Returns a [`Val::Auto`](../../prelude/enum.Val.html#variant.Auto "variant bevy::prelude::Val::Auto") where the value is automatically determined based on the context and other [`Node`](../../prelude/struct.Node.html "struct bevy::prelude::Node") properties.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/3d/tonemapping.rs ([line 188](../../../src/tonemapping/tonemapping.rs.html#188))

```rust
155fn setup_image_viewer_scene(
156    mut commands: Commands,
157    mut meshes: ResMut<Assets<Mesh>>,
158    mut materials: ResMut<Assets<StandardMaterial>>,
159    camera_transform: Res<CameraTransform>,
160) {
161    let mut transform = camera_transform.0;
162    transform.translation += *transform.forward();
163
164    // exr/hdr viewer (exr requires enabling bevy feature)
165    commands.spawn((
166        Mesh3d(meshes.add(Rectangle::default())),
167        MeshMaterial3d(materials.add(StandardMaterial {
168            base_color_texture: None,
169            unlit: true,
170            ..default()
171        })),
172        transform,
173        Visibility::Hidden,
174        SceneNumber(3),
175        HDRViewer,
176    ));
177
178    commands.spawn((
179        Text::new("Drag and drop an HDR or EXR file"),
180        TextFont {
181            font_size: FontSize::Px(36.0),
182            ..default()
183        },
184        TextColor(Color::BLACK),
185        TextLayout::justify(Justify::Center),
186        Node {
187            align_self: AlignSelf::Center,
188            margin: UiRect::all(auto()),
189            ..default()
190        },
191        SceneNumber(3),
192        Visibility::Hidden,
193    ));
194}
```

Hide additional examples

examples/ui/text/text\_input.rs ([line 56](../../../src/text_input/text_input.rs.html#56))

```rust
47fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
48    commands.spawn(Camera2d);
49
50    let root = commands
51        .spawn(Node {
52            align_items: AlignItems::Center,
53            flex_direction: FlexDirection::Column,
54            padding: px(20).all(),
55            row_gap: px(16),
56            margin: auto().all(),
57            ..default()
58        })
59        .id();
60
61    let text_instructions = commands
62        .spawn((
63            Text::new("Enter to submit text\nTab to switch inputs"),
64            TextFont {
65                font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
66                font_size: FontSize::Px(25.0),
67                ..default()
68            },
69        ))
70        .id();
71
72    let text_input_left = build_input_text(&mut commands, true, 24.0);
73    let text_input_right = build_input_text(&mut commands, false, 24.0);
74
75    let input_container = commands
76        .spawn((
77            Node {
78                column_gap: px(16),
79                ..default()
80            },
81            AutoFocus,
82            TabGroup::new(0),
83        ))
84        .id();
85
86    // Set up a text output to see the result of our text input
87    let text_output = commands
88        .spawn((
89            Node {
90                width: px(400),
91                border: px(2).all(),
92                padding: px(8).all(),
93                ..Default::default()
94            },
95            BorderColor::from(Color::from(SLATE_300)),
96            Text::new(""),
97            TextOutput,
98            TextLayout {
99                linebreak: LineBreak::WordOrCharacter,
100                ..default()
101            },
102            TextFont {
103                font_size: FontSize::Px(24.0),
104                ..default()
105            },
106        ))
107        .id();
108
109    commands
110        .entity(input_container)
111        .add_children(&[text_input_left, text_input_right]);
112
113    commands
114        .entity(root)
115        .add_children(&[text_instructions, input_container, text_output]);
116}
```

examples/animation/animation\_masks.rs ([line 181](../../../src/animation_masks/animation_masks.rs.html#181))

```rust
158fn setup_ui(mut commands: Commands) {
159    // Add help text.
160    commands.spawn((
161        Text::new("Click on a button to toggle animations for its associated bones"),
162        Node {
163            position_type: PositionType::Absolute,
164            left: px(12),
165            top: px(12),
166            ..default()
167        },
168    ));
169
170    // Add the buttons that allow the user to toggle mask groups on and off.
171    commands.spawn((
172        Node {
173            flex_direction: FlexDirection::Column,
174            position_type: PositionType::Absolute,
175            row_gap: px(6),
176            left: px(12),
177            bottom: px(12),
178            ..default()
179        },
180        children![
181            new_mask_group_control("Head", auto(), MASK_GROUP_HEAD),
182            (
183                Node {
184                    flex_direction: FlexDirection::Row,
185                    column_gap: px(6),
186                    ..default()
187                },
188                children![
189                    new_mask_group_control(
190                        "Left Front Leg",
191                        px(MASK_GROUP_BUTTON_WIDTH),
192                        MASK_GROUP_LEFT_FRONT_LEG,
193                    ),
194                    new_mask_group_control(
195                        "Right Front Leg",
196                        px(MASK_GROUP_BUTTON_WIDTH),
197                        MASK_GROUP_RIGHT_FRONT_LEG,
198                    )
199                ],
200            ),
201            (
202                Node {
203                    flex_direction: FlexDirection::Row,
204                    column_gap: px(6),
205                    ..default()
206                },
207                children![
208                    new_mask_group_control(
209                        "Left Hind Leg",
210                        px(MASK_GROUP_BUTTON_WIDTH),
211                        MASK_GROUP_LEFT_HIND_LEG,
212                    ),
213                    new_mask_group_control(
214                        "Right Hind Leg",
215                        px(MASK_GROUP_BUTTON_WIDTH),
216                        MASK_GROUP_RIGHT_HIND_LEG,
217                    )
218                ]
219            ),
220            new_mask_group_control("Tail", auto(), MASK_GROUP_TAIL),
221        ],
222    ));
223}
```

examples/ui/layout/size\_constraints.rs ([line 186](../../../src/size_constraints/size_constraints.rs.html#186))

```rust
139fn spawn_button_row(
140    parent: &mut ChildSpawnerCommands,
141    constraint: Constraint,
142    text_style: (TextFont, TextColor),
143) {
144    let label = match constraint {
145        Constraint::FlexBasis => "flex_basis",
146        Constraint::Width => "size",
147        Constraint::MinWidth => "min_size",
148        Constraint::MaxWidth => "max_size",
149    };
150
151    parent
152        .spawn((
153            Node {
154                flex_direction: FlexDirection::Column,
155                padding: UiRect::all(px(2)),
156                align_items: AlignItems::Stretch,
157                ..default()
158            },
159            BackgroundColor(Color::BLACK),
160        ))
161        .with_children(|parent| {
162            parent
163                .spawn(Node {
164                    flex_direction: FlexDirection::Row,
165                    justify_content: JustifyContent::End,
166                    padding: UiRect::all(px(2)),
167                    ..default()
168                })
169                .with_children(|parent| {
170                    // spawn row label
171                    parent
172                        .spawn((Node {
173                            min_width: px(200),
174                            max_width: px(200),
175                            justify_content: JustifyContent::Center,
176                            align_items: AlignItems::Center,
177                            ..default()
178                        },))
179                        .with_child((Text::new(label), text_style.clone()));
180
181                    // spawn row buttons
182                    parent.spawn(Node::default()).with_children(|parent| {
183                        spawn_button(
184                            parent,
185                            constraint,
186                            ButtonValue(auto()),
187                            "Auto".to_string(),
188                            text_style.clone(),
189                            true,
190                        );
191                        for percent_value in [0, 25, 50, 75, 100, 125] {
192                            spawn_button(
193                                parent,
194                                constraint,
195                                ButtonValue(percent(percent_value)),
196                                format!("{percent_value}%"),
197                                text_style.clone(),
198                                false,
199                            );
200                        }
201                    });
202                });
203        });
204}
```

examples/ui/layout/anchor\_layout.rs ([line 35](../../../src/anchor_layout/anchor_layout.rs.html#35))

```rust
19fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
20    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
21    commands.spawn(Camera2d);
22
23    let rows = [
24        (
25            "left: 10px\ntop: 10px",
26            Node {
27                left: px(10),
28                top: px(10),
29                ..default()
30            },
31        ),
32        (
33            "center: 10px\ntop: 10px",
34            Node {
35                margin: auto().horizontal(),
36                top: px(10),
37                ..default()
38            },
39        ),
40        (
41            "right: 10px\ntop: 10px",
42            Node {
43                right: px(10),
44                top: px(10),
45                ..default()
46            },
47        ),
48        (
49            "left: 10px\ncenter: 10px",
50            Node {
51                left: px(10),
52                margin: UiRect::vertical(auto()),
53                ..default()
54            },
55        ),
56        (
57            "center: 10px\ncenter: 10px",
58            Node {
59                margin: UiRect::all(auto()),
60                ..default()
61            },
62        ),
63        (
64            "right: 10px\ncenter: 10px",
65            Node {
66                right: px(10),
67                margin: UiRect::vertical(auto()),
68                ..default()
69            },
70        ),
71        (
72            "left: 10px\nbottom: 10px",
73            Node {
74                left: px(10),
75                bottom: px(10),
76                ..default()
77            },
78        ),
79        (
80            "center: 10px\nbottom: 10px",
81            Node {
82                margin: UiRect::horizontal(auto()),
83                bottom: px(10),
84                ..default()
85            },
86        ),
87        (
88            "right: 10px\nbottom: 10px",
89            Node {
90                right: px(10),
91                bottom: px(10),
92                ..default()
93            },
94        ),
95    ];
96
97    // let font = font.clone();
98    commands.spawn((
99        Node {
100            // fill the entire window
101            width: percent(100),
102            height: percent(100),
103            padding: MARGIN.all(),
104            row_gap: MARGIN,
105            column_gap: MARGIN,
106            display: Display::Grid,
107            grid_template_columns: RepeatedGridTrack::fr(3, 1.),
108            grid_template_rows: RepeatedGridTrack::fr(3, 1.),
109            ..default()
110        },
111        BackgroundColor(Color::BLACK),
112        Children::spawn(SpawnIter(
113            rows.into_iter()
114                .map(move |v| anchored_node(font.clone(), v.1, v.0)),
115        )),
116    ));
117}
```

examples/ui/render\_ui\_to\_texture.rs ([line 95](../../../src/render_ui_to_texture/render_ui_to_texture.rs.html#95))

```rust
35fn setup(
36    mut commands: Commands,
37    mut meshes: ResMut<Assets<Mesh>>,
38    mut materials: ResMut<Assets<StandardMaterial>>,
39    mut images: ResMut<Assets<Image>>,
40) {
41    let size = Extent3d {
42        width: 512,
43        height: 512,
44        ..default()
45    };
46
47    // This is the texture that will be rendered to.
48    let mut image = Image::new_fill(
49        size,
50        TextureDimension::D2,
51        &[0, 0, 0, 0],
52        TextureFormat::Bgra8UnormSrgb,
53        RenderAssetUsages::default(),
54    );
55    // You need to set these texture usage flags in order to use the image as a render target
56    image.texture_descriptor.usage =
57        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
58
59    let image_handle = images.add(image);
60
61    // Light
62    commands.spawn(DirectionalLight::default());
63
64    let texture_camera = commands
65        .spawn((
66            Camera2d,
67            Camera {
68                // render before the "main pass" camera
69                order: -1,
70                ..default()
71            },
72            RenderTarget::Image(image_handle.clone().into()),
73        ))
74        .id();
75
76    commands
77        .spawn((
78            Node {
79                // Cover the whole image
80                width: percent(100),
81                height: percent(100),
82                flex_direction: FlexDirection::Column,
83                justify_content: JustifyContent::Center,
84                align_items: AlignItems::Center,
85                ..default()
86            },
87            BackgroundColor(GRAY.into()),
88            UiTargetCamera(texture_camera),
89        ))
90        .with_children(|parent| {
91            parent
92                .spawn((
93                    Node {
94                        position_type: PositionType::Absolute,
95                        width: auto(),
96                        height: auto(),
97                        align_items: AlignItems::Center,
98                        padding: UiRect::all(px(20.)),
99                        border_radius: BorderRadius::all(px(10.)),
100                        ..default()
101                    },
102                    BackgroundColor(BLUE.into()),
103                ))
104                .observe(
105                    |drag: On<Pointer<Drag>>, mut nodes: Query<(&mut Node, &ComputedNode)>| {
106                        let (mut node, computed) = nodes.get_mut(drag.entity).unwrap();
107                        node.left = px(drag.pointer_location.position.x - computed.size.x / 2.0);
108                        node.top = px(drag.pointer_location.position.y - 50.0);
109                    },
110                )
111                .observe(
112                    |over: On<Pointer<Over>>, mut colors: Query<&mut BackgroundColor>| {
113                        colors.get_mut(over.entity).unwrap().0 = RED.into();
114                    },
115                )
116                .observe(
117                    |out: On<Pointer<Out>>, mut colors: Query<&mut BackgroundColor>| {
118                        colors.get_mut(out.entity).unwrap().0 = BLUE.into();
119                    },
120                )
121                .with_children(|parent| {
122                    parent.spawn((
123                        Text::new("Drag Me!"),
124                        TextFont {
125                            font_size: FontSize::Px(40.0),
126                            ..default()
127                        },
128                        TextColor::WHITE,
129                    ));
130                });
131        });
132
133    let mesh_handle = meshes.add(Cuboid::default());
134
135    // This material has the texture that has been rendered.
136    let material_handle = materials.add(StandardMaterial {
137        base_color_texture: Some(image_handle),
138        reflectance: 0.02,
139        unlit: false,
140        ..default()
141    });
142
143    // Cube with material containing the rendered UI texture.
144    commands.spawn((
145        Mesh3d(mesh_handle),
146        MeshMaterial3d(material_handle),
147        Transform::from_xyz(0.0, 0.0, 1.5).with_rotation(Quat::from_rotation_x(PI)),
148        Cube,
149    ));
150
151    // The main pass camera.
152    commands.spawn((
153        Camera3d::default(),
154        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
155    ));
156
157    commands.spawn(CUBE_POINTER_ID);
158}
```

Additional examples can be found in:  

*   [examples/ui/layout/grid.rs](../../../src/grid/grid.rs.html#164)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#1859)