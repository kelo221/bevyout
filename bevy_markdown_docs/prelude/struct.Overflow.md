[bevy](../index.html)::[prelude](index.html)

# Struct Overflow 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1242)

```rust
pub struct Overflow {
    pub x: OverflowAxis,
    pub y: OverflowAxis,
}
```

Whether to show or hide overflowing items

## Fields

`x: [OverflowAxis](enum.OverflowAxis.html "enum bevy::prelude::OverflowAxis")`

Whether to show or clip overflowing items on the x axis

`y: [OverflowAxis](enum.OverflowAxis.html "enum bevy::prelude::OverflowAxis")`

Whether to show or clip overflowing items on the y axis

## Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1249)

### impl [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1250)

#### pub const [DEFAULT](#associatedconstant.DEFAULT): [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1256)

#### pub const fn [visible](#method.visible)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Show overflowing items on both axes

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/overflow\_debug.rs ([line 254](../../src/overflow_debug/overflow_debug.rs.html#254))

```rust
235fn toggle_overflow(
236    mut containers: Query<&mut Node, With<Container>>,
237    instructions: Single<Entity, With<Instructions>>,
238    mut writer: TextUiWriter,
239) {
240    for mut node in &mut containers {
241        node.overflow = match node.overflow {
242            Overflow {
243                x: OverflowAxis::Visible,
244                y: OverflowAxis::Visible,
245            } => Overflow::clip_y(),
246            Overflow {
247                x: OverflowAxis::Visible,
248                y: OverflowAxis::Clip,
249            } => Overflow::clip_x(),
250            Overflow {
251                x: OverflowAxis::Clip,
252                y: OverflowAxis::Visible,
253            } => Overflow::clip(),
254            _ => Overflow::visible(),
255        };
256
257        let entity = *instructions;
258        *writer.text(entity, 1) = format!("{:?}", node.overflow);
259    }
260}
```

Hide additional examples

examples/testbed/ui.rs ([line 980](../../src/testbed_ui/ui.rs.html#980))

```rust
962    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
963        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Overflow)));
964        let image = asset_server.load("branding/icon.png");
965
966        commands
967            .spawn((
968                Node {
969                    width: percent(100),
970                    height: percent(100),
971                    align_items: AlignItems::Center,
972                    justify_content: JustifyContent::SpaceAround,
973                    ..Default::default()
974                },
975                BackgroundColor(BLUE.into()),
976                DespawnOnExit(super::Scene::Overflow),
977            ))
978            .with_children(|parent| {
979                for overflow in [
980                    Overflow::visible(),
981                    Overflow::clip_x(),
982                    Overflow::clip_y(),
983                    Overflow::clip(),
984                ] {
985                    parent
986                        .spawn((
987                            Node {
988                                width: px(100),
989                                height: px(100),
990                                padding: UiRect {
991                                    left: px(25),
992                                    top: px(25),
993                                    ..Default::default()
994                                },
995                                border: UiRect::all(px(5)),
996                                overflow,
997                                ..default()
998                            },
999                            BorderColor::all(RED),
1000                            BackgroundColor(Color::WHITE),
1001                        ))
1002                        .with_children(|parent| {
1003                            parent.spawn((
1004                                ImageNode::new(image.clone()),
1005                                Node {
1006                                    min_width: px(100),
1007                                    min_height: px(100),
1008                                    ..default()
1009                                },
1010                                Interaction::default(),
1011                                Outline {
1012                                    width: px(2),
1013                                    offset: px(2),
1014                                    color: Color::NONE,
1015                                },
1016                            ));
1017                        });
1018                }
1019            });
1020    }
```

examples/ui/scroll\_and\_overflow/overflow.rs ([line 33](../../src/overflow/overflow.rs.html#33))

```rust
13fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
14    commands.spawn(Camera2d);
15
16    let text_style = TextFont::default();
17
18    let image = asset_server.load("branding/icon.png");
19
20    commands
21        .spawn((
22            Node {
23                width: percent(100),
24                height: percent(100),
25                align_items: AlignItems::Center,
26                justify_content: JustifyContent::Center,
27                ..Default::default()
28            },
29            BackgroundColor(ANTIQUE_WHITE.into()),
30        ))
31        .with_children(|parent| {
32            for overflow in [
33                Overflow::visible(),
34                Overflow::clip_x(),
35                Overflow::clip_y(),
36                Overflow::clip(),
37            ] {
38                parent
39                    .spawn(Node {
40                        flex_direction: FlexDirection::Column,
41                        align_items: AlignItems::Center,
42                        margin: UiRect::horizontal(px(25)),
43                        ..Default::default()
44                    })
45                    .with_children(|parent| {
46                        let label = format!("{overflow:#?}");
47                        parent
48                            .spawn((
49                                Node {
50                                    padding: UiRect::all(px(10)),
51                                    margin: UiRect::bottom(px(25)),
52                                    ..Default::default()
53                                },
54                                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
55                            ))
56                            .with_children(|parent| {
57                                parent.spawn((Text::new(label), text_style.clone()));
58                            });
59                        parent
60                            .spawn((
61                                Node {
62                                    width: px(100),
63                                    height: px(100),
64                                    padding: UiRect {
65                                        left: px(25),
66                                        top: px(25),
67                                        ..Default::default()
68                                    },
69                                    border: UiRect::all(px(5)),
70                                    overflow,
71                                    ..default()
72                                },
73                                BorderColor::all(Color::BLACK),
74                                BackgroundColor(GRAY.into()),
75                            ))
76                            .with_children(|parent| {
77                                parent.spawn((
78                                    ImageNode::new(image.clone()),
79                                    Node {
80                                        min_width: px(100),
81                                        min_height: px(100),
82                                        ..default()
83                                    },
84                                    Interaction::default(),
85                                    Outline {
86                                        width: px(2),
87                                        offset: px(2),
88                                        color: Color::NONE,
89                                    },
90                                ));
91                            });
92                    });
93            }
94        });
95}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1264)

#### pub const fn [clip](#method.clip)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Clip overflowing items on both axes

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/overflow\_debug.rs ([line 100](../../src/overflow_debug/overflow_debug.rs.html#100))

```rust
76fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
77    // Camera
78
79    commands.spawn(Camera2d);
80
81    // Instructions
82
83    let text_font = TextFont::default();
84
85    commands
86        .spawn((
87            Text::new(
88                "Next Overflow Setting (O)\nNext Container Size (S)\nToggle Animation (space)\n\n",
89            ),
90            text_font.clone(),
91            Node {
92                position_type: PositionType::Absolute,
93                top: px(12),
94                left: px(12),
95                ..default()
96            },
97            Instructions,
98        ))
99        .with_child((
100            TextSpan::new(format!("{:?}", Overflow::clip())),
101            text_font.clone(),
102        ));
103
104    // Overflow Debug
105
106    commands
107        .spawn(Node {
108            width: percent(100),
109            height: percent(100),
110            justify_content: JustifyContent::Center,
111            align_items: AlignItems::Center,
112            ..default()
113        })
114        .with_children(|parent| {
115            parent
116                .spawn(Node {
117                    display: Display::Grid,
118                    grid_template_columns: RepeatedGridTrack::px(3, CONTAINER_SIZE),
119                    grid_template_rows: RepeatedGridTrack::px(2, CONTAINER_SIZE),
120                    row_gap: px(80),
121                    column_gap: px(80),
122                    ..default()
123                })
124                .with_children(|parent| {
125                    spawn_image(parent, &asset_server, Move);
126                    spawn_image(parent, &asset_server, Scale);
127                    spawn_image(parent, &asset_server, Rotate);
128
129                    spawn_text(parent, &asset_server, Move);
130                    spawn_text(parent, &asset_server, Scale);
131                    spawn_text(parent, &asset_server, Rotate);
132                });
133        });
134}
135
136fn spawn_image(
137    parent: &mut ChildSpawnerCommands,
138    asset_server: &Res<AssetServer>,
139    update_transform: impl UpdateTransform + Component,
140) {
141    spawn_container(parent, update_transform, |parent| {
142        parent.spawn((
143            ImageNode::new(asset_server.load("branding/bevy_logo_dark_big.png")),
144            Node {
145                height: px(100),
146                position_type: PositionType::Absolute,
147                top: px(-50),
148                left: px(-200),
149                ..default()
150            },
151        ));
152    });
153}
154
155fn spawn_text(
156    parent: &mut ChildSpawnerCommands,
157    asset_server: &Res<AssetServer>,
158    update_transform: impl UpdateTransform + Component,
159) {
160    spawn_container(parent, update_transform, |parent| {
161        parent.spawn((
162            Text::new("Bevy"),
163            TextFont {
164                font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
165                font_size: FontSize::Px(100.0),
166                ..default()
167            },
168        ));
169    });
170}
171
172fn spawn_container(
173    parent: &mut ChildSpawnerCommands,
174    update_transform: impl UpdateTransform + Component,
175    spawn_children: impl FnOnce(&mut ChildSpawnerCommands),
176) {
177    parent
178        .spawn((
179            Node {
180                width: percent(100),
181                height: percent(100),
182                align_items: AlignItems::Center,
183                justify_content: JustifyContent::Center,
184                overflow: Overflow::clip(),
185                ..default()
186            },
187            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
188            Container(0),
189        ))
190        .with_children(|parent| {
191            parent
192                .spawn((
193                    Node {
194                        align_items: AlignItems::Center,
195                        justify_content: JustifyContent::Center,
196                        ..default()
197                    },
198                    update_transform,
199                ))
200                .with_children(spawn_children);
201        });
202}
203
204fn update_animation(
205    mut animation: ResMut<AnimationState>,
206    time: Res<Time>,
207    keys: Res<ButtonInput<KeyCode>>,
208) {
209    let delta = time.elapsed_secs();
210
211    if keys.just_pressed(KeyCode::Space) {
212        animation.playing = !animation.playing;
213
214        if !animation.playing {
215            animation.paused_at = delta;
216        } else {
217            animation.paused_total += delta - animation.paused_at;
218        }
219    }
220
221    if animation.playing {
222        animation.t = (delta - animation.paused_total) % LOOP_LENGTH / LOOP_LENGTH;
223    }
224}
225
226fn update_transform<T: UpdateTransform + Component>(
227    animation: Res<AnimationState>,
228    mut containers: Query<(&mut UiTransform, &T)>,
229) {
230    for (mut transform, update_transform) in &mut containers {
231        update_transform.update(animation.t, &mut transform);
232    }
233}
234
235fn toggle_overflow(
236    mut containers: Query<&mut Node, With<Container>>,
237    instructions: Single<Entity, With<Instructions>>,
238    mut writer: TextUiWriter,
239) {
240    for mut node in &mut containers {
241        node.overflow = match node.overflow {
242            Overflow {
243                x: OverflowAxis::Visible,
244                y: OverflowAxis::Visible,
245            } => Overflow::clip_y(),
246            Overflow {
247                x: OverflowAxis::Visible,
248                y: OverflowAxis::Clip,
249            } => Overflow::clip_x(),
250            Overflow {
251                x: OverflowAxis::Clip,
252                y: OverflowAxis::Visible,
253            } => Overflow::clip(),
254            _ => Overflow::visible(),
255        };
256
257        let entity = *instructions;
258        *writer.text(entity, 1) = format!("{:?}", node.overflow);
259    }
260}
```

Hide additional examples

examples/testbed/ui.rs ([line 983](../../src/testbed_ui/ui.rs.html#983))

```rust
962    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
963        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Overflow)));
964        let image = asset_server.load("branding/icon.png");
965
966        commands
967            .spawn((
968                Node {
969                    width: percent(100),
970                    height: percent(100),
971                    align_items: AlignItems::Center,
972                    justify_content: JustifyContent::SpaceAround,
973                    ..Default::default()
974                },
975                BackgroundColor(BLUE.into()),
976                DespawnOnExit(super::Scene::Overflow),
977            ))
978            .with_children(|parent| {
979                for overflow in [
980                    Overflow::visible(),
981                    Overflow::clip_x(),
982                    Overflow::clip_y(),
983                    Overflow::clip(),
984                ] {
985                    parent
986                        .spawn((
987                            Node {
988                                width: px(100),
989                                height: px(100),
990                                padding: UiRect {
991                                    left: px(25),
992                                    top: px(25),
993                                    ..Default::default()
994                                },
995                                border: UiRect::all(px(5)),
996                                overflow,
997                                ..default()
998                            },
999                            BorderColor::all(RED),
1000                            BackgroundColor(Color::WHITE),
1001                        ))
1002                        .with_children(|parent| {
1003                            parent.spawn((
1004                                ImageNode::new(image.clone()),
1005                                Node {
1006                                    min_width: px(100),
1007                                    min_height: px(100),
1008                                    ..default()
1009                                },
1010                                Interaction::default(),
1011                                Outline {
1012                                    width: px(2),
1013                                    offset: px(2),
1014                                    color: Color::NONE,
1015                                },
1016                            ));
1017                        });
1018                }
1019            });
1020    }
1021}
1022
1023mod slice {
1024    use bevy::prelude::*;
1025
1026    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
1027        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Slice)));
1028        let image = asset_server.load("textures/fantasy_ui_borders/numbered_slices.png");
1029
1030        let slicer = TextureSlicer {
1031            border: BorderRect::all(16.0),
1032            center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
1033            sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
1034            ..default()
1035        };
1036        commands
1037            .spawn((
1038                Node {
1039                    width: percent(100),
1040                    height: percent(100),
1041                    flex_direction: FlexDirection::Column,
1042                    justify_content: JustifyContent::SpaceAround,
1043                    align_content: AlignContent::Center,
1044                    ..default()
1045                },
1046                DespawnOnExit(super::Scene::Slice),
1047            ))
1048            .with_children(|parent| {
1049                for visual_box in [
1050                    VisualBox::BorderBox,
1051                    VisualBox::PaddingBox,
1052                    VisualBox::ContentBox,
1053                ] {
1054                    parent
1055                        .spawn(Node {
1056                            justify_content: JustifyContent::SpaceAround,
1057                            ..default()
1058                        })
1059                        .with_children(|parent| {
1060                            for [w, h] in [[200.0, 200.0], [300.0, 200.0], [150., 200.0]] {
1061                                parent.spawn((
1062                                    Button,
1063                                    ImageNode {
1064                                        image: image.clone(),
1065                                        image_mode: NodeImageMode::Sliced(slicer.clone()),
1066                                        visual_box,
1067                                        ..default()
1068                                    },
1069                                    Node {
1070                                        width: px(w),
1071                                        height: px(h),
1072                                        border: px(20.).all(),
1073                                        padding: px(20.).all(),
1074                                        ..default()
1075                                    },
1076                                    Outline {
1077                                        width: px(2.),
1078                                        ..default()
1079                                    },
1080                                ));
1081                            }
1082
1083                            parent.spawn((
1084                                ImageNode {
1085                                    image: asset_server
1086                                        .load("textures/fantasy_ui_borders/panel-border-010.png"),
1087                                    image_mode: NodeImageMode::Sliced(TextureSlicer {
1088                                        border: BorderRect::all(22.0),
1089                                        center_scale_mode: SliceScaleMode::Stretch,
1090                                        sides_scale_mode: SliceScaleMode::Stretch,
1091                                        max_corner_scale: 1.0,
1092                                    }),
1093                                    visual_box,
1094                                    ..Default::default()
1095                                },
1096                                Node {
1097                                    width: px(200),
1098                                    height: px(200),
1099                                    border: px(20.).all(),
1100                                    padding: px(20.).all(),
1101                                    ..default()
1102                                },
1103                                Outline {
1104                                    color: bevy::color::palettes::css::DARK_CYAN.into(),
1105                                    width: px(2.),
1106                                    ..default()
1107                                },
1108                                BackgroundColor(bevy::color::palettes::css::NAVY.into()),
1109                            ));
1110                        });
1111                }
1112            });
1113    }
1114}
1115
1116mod layout_rounding {
1117    use bevy::{color::palettes::css::*, prelude::*};
1118
1119    pub fn setup(mut commands: Commands) {
1120        commands.spawn((Camera2d, DespawnOnExit(super::Scene::LayoutRounding)));
1121
1122        commands
1123            .spawn((
1124                Node {
1125                    display: Display::Grid,
1126                    width: percent(100),
1127                    height: percent(100),
1128                    grid_template_rows: vec![RepeatedGridTrack::fr(10, 1.)],
1129                    ..Default::default()
1130                },
1131                BackgroundColor(Color::WHITE),
1132                DespawnOnExit(super::Scene::LayoutRounding),
1133            ))
1134            .with_children(|commands| {
1135                for i in 2..12 {
1136                    commands
1137                        .spawn(Node {
1138                            display: Display::Grid,
1139                            grid_template_columns: vec![RepeatedGridTrack::fr(i, 1.)],
1140                            ..Default::default()
1141                        })
1142                        .with_children(|commands| {
1143                            for _ in 0..i {
1144                                commands.spawn((
1145                                    Node {
1146                                        border: UiRect::all(px(5)),
1147                                        ..Default::default()
1148                                    },
1149                                    BackgroundColor(MAROON.into()),
1150                                    BorderColor::all(DARK_BLUE),
1151                                ));
1152                            }
1153                        });
1154                }
1155            });
1156    }
1157}
1158
1159mod linear_gradient {
1160    use bevy::camera::Camera2d;
1161    use bevy::color::palettes::css::BLUE;
1162    use bevy::color::palettes::css::LIME;
1163    use bevy::color::palettes::css::RED;
1164    use bevy::color::palettes::css::YELLOW;
1165    use bevy::color::Color;
1166    use bevy::ecs::prelude::*;
1167    use bevy::state::state_scoped::DespawnOnExit;
1168    use bevy::text::TextFont;
1169    use bevy::ui::AlignItems;
1170    use bevy::ui::BackgroundGradient;
1171    use bevy::ui::ColorStop;
1172    use bevy::ui::GridPlacement;
1173    use bevy::ui::InterpolationColorSpace;
1174    use bevy::ui::JustifyContent;
1175    use bevy::ui::LinearGradient;
1176    use bevy::ui::Node;
1177    use bevy::ui::PositionType;
1178    use bevy::utils::default;
1179
1180    pub fn setup(mut commands: Commands) {
1181        commands.spawn((Camera2d, DespawnOnExit(super::Scene::LinearGradient)));
1182        commands
1183            .spawn((
1184                Node {
1185                    flex_direction: bevy::ui::FlexDirection::Column,
1186                    width: bevy::ui::percent(100),
1187                    height: bevy::ui::percent(100),
1188                    justify_content: JustifyContent::Center,
1189                    align_items: AlignItems::Center,
1190                    row_gap: bevy::ui::px(5),
1191                    ..default()
1192                },
1193                DespawnOnExit(super::Scene::LinearGradient),
1194            ))
1195            .with_children(|commands| {
1196                let mut i = 0;
1197                commands
1198                    .spawn(Node {
1199                        display: bevy::ui::Display::Grid,
1200                        row_gap: bevy::ui::px(4),
1201                        column_gap: bevy::ui::px(4),
1202                        ..Default::default()
1203                    })
1204                    .with_children(|commands| {
1205                        for stops in [
1206                            vec![ColorStop::auto(RED), ColorStop::auto(YELLOW)],
1207                            vec![
1208                                ColorStop::auto(Color::BLACK),
1209                                ColorStop::auto(RED),
1210                                ColorStop::auto(Color::WHITE),
1211                            ],
1212                            vec![
1213                                Color::hsl(180.71191, 0.0, 0.3137255).into(),
1214                                Color::hsl(180.71191, 0.5, 0.3137255).into(),
1215                                Color::hsl(180.71191, 1.0, 0.3137255).into(),
1216                            ],
1217                            vec![
1218                                Color::hsl(180.71191, 0.825, 0.0).into(),
1219                                Color::hsl(180.71191, 0.825, 0.5).into(),
1220                                Color::hsl(180.71191, 0.825, 1.0).into(),
1221                            ],
1222                            vec![
1223                                Color::hsl(0.0 + 0.0001, 1.0, 0.5).into(),
1224                                Color::hsl(180.0, 1.0, 0.5).into(),
1225                                Color::hsl(360.0 - 0.0001, 1.0, 0.5).into(),
1226                            ],
1227                            vec![
1228                                Color::WHITE.into(),
1229                                RED.into(),
1230                                LIME.into(),
1231                                BLUE.into(),
1232                                Color::BLACK.into(),
1233                            ],
1234                        ] {
1235                            for color_space in [
1236                                InterpolationColorSpace::LinearRgba,
1237                                InterpolationColorSpace::Srgba,
1238                                InterpolationColorSpace::Oklaba,
1239                                InterpolationColorSpace::Oklcha,
1240                                InterpolationColorSpace::OklchaLong,
1241                                InterpolationColorSpace::Hsla,
1242                                InterpolationColorSpace::HslaLong,
1243                                InterpolationColorSpace::Hsva,
1244                                InterpolationColorSpace::HsvaLong,
1245                            ] {
1246                                let row = i % 18 + 1;
1247                                let column = i / 18 + 1;
1248                                i += 1;
1249
1250                                commands.spawn((
1251                                    Node {
1252                                        grid_row: GridPlacement::start(row as i16 + 1),
1253                                        grid_column: GridPlacement::start(column as i16 + 1),
1254                                        justify_content: JustifyContent::SpaceEvenly,
1255                                        ..Default::default()
1256                                    },
1257                                    children![(
1258                                        Node {
1259                                            height: bevy::ui::px(30),
1260                                            width: bevy::ui::px(300),
1261                                            justify_content: JustifyContent::Center,
1262                                            ..Default::default()
1263                                        },
1264                                        BackgroundGradient::from(LinearGradient {
1265                                            color_space,
1266                                            angle: LinearGradient::TO_RIGHT,
1267                                            stops: stops.clone(),
1268                                        }),
1269                                        children![
1270                                            Node {
1271                                                position_type: PositionType::Absolute,
1272                                                ..default()
1273                                            },
1274                                            TextFont::from_font_size(10.),
1275                                            bevy::ui::widget::Text(format!("{color_space:?}")),
1276                                        ]
1277                                    )],
1278                                ));
1279                            }
1280                        }
1281                    });
1282            });
1283    }
1284}
1285
1286mod radial_gradient {
1287    use bevy::color::palettes::css::RED;
1288    use bevy::color::palettes::tailwind::GRAY_700;
1289    use bevy::prelude::*;
1290    use bevy::ui::ColorStop;
1291
1292    const CELL_SIZE: f32 = 80.;
1293    const GAP: f32 = 10.;
1294
1295    pub fn setup(mut commands: Commands) {
1296        let color_stops = vec![
1297            ColorStop::new(Color::BLACK, px(5)),
1298            ColorStop::new(Color::WHITE, px(5)),
1299            ColorStop::new(Color::WHITE, percent(100)),
1300            ColorStop::auto(RED),
1301        ];
1302
1303        commands.spawn((Camera2d, DespawnOnExit(super::Scene::RadialGradient)));
1304        commands
1305            .spawn((
1306                Node {
1307                    width: percent(100),
1308                    height: percent(100),
1309                    display: Display::Grid,
1310                    align_items: AlignItems::Start,
1311                    grid_template_columns: vec![RepeatedGridTrack::px(
1312                        GridTrackRepetition::AutoFill,
1313                        CELL_SIZE,
1314                    )],
1315                    grid_auto_flow: GridAutoFlow::Row,
1316                    row_gap: px(GAP),
1317                    column_gap: px(GAP),
1318                    padding: UiRect::all(px(GAP)),
1319                    ..default()
1320                },
1321                DespawnOnExit(super::Scene::RadialGradient),
1322            ))
1323            .with_children(|commands| {
1324                for (shape, shape_label) in [
1325                    (RadialGradientShape::ClosestSide, "ClosestSide"),
1326                    (RadialGradientShape::FarthestSide, "FarthestSide"),
1327                    (RadialGradientShape::Circle(percent(55)), "Circle(55%)"),
1328                    (RadialGradientShape::FarthestCorner, "FarthestCorner"),
1329                ] {
1330                    for (position, position_label) in [
1331                        (UiPosition::TOP_LEFT, "TOP_LEFT"),
1332                        (UiPosition::LEFT, "LEFT"),
1333                        (UiPosition::BOTTOM_LEFT, "BOTTOM_LEFT"),
1334                        (UiPosition::TOP, "TOP"),
1335                        (UiPosition::CENTER, "CENTER"),
1336                        (UiPosition::BOTTOM, "BOTTOM"),
1337                        (UiPosition::TOP_RIGHT, "TOP_RIGHT"),
1338                        (UiPosition::RIGHT, "RIGHT"),
1339                        (UiPosition::BOTTOM_RIGHT, "BOTTOM_RIGHT"),
1340                    ] {
1341                        for (w, h) in [(CELL_SIZE, CELL_SIZE), (CELL_SIZE, CELL_SIZE / 2.)] {
1342                            commands
1343                                .spawn((
1344                                    BackgroundColor(GRAY_700.into()),
1345                                    Node {
1346                                        display: Display::Grid,
1347                                        width: px(CELL_SIZE),
1348                                        ..Default::default()
1349                                    },
1350                                ))
1351                                .with_children(|commands| {
1352                                    commands.spawn((
1353                                        Node {
1354                                            margin: UiRect::all(px(2)),
1355                                            ..default()
1356                                        },
1357                                        Text(format!("{shape_label}\n{position_label}")),
1358                                        TextFont::from_font_size(9.),
1359                                    ));
1360                                    commands.spawn((
1361                                        Node {
1362                                            width: px(w),
1363                                            height: px(h),
1364                                            ..default()
1365                                        },
1366                                        BackgroundGradient::from(RadialGradient {
1367                                            stops: color_stops.clone(),
1368                                            position,
1369                                            shape,
1370                                            ..default()
1371                                        }),
1372                                    ));
1373                                });
1374                        }
1375                    }
1376                }
1377            });
1378    }
1379}
1380
1381mod transformations {
1382    use bevy::{color::palettes::css::*, prelude::*};
1383
1384    pub fn setup(mut commands: Commands) {
1385        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Transformations)));
1386        commands
1387            .spawn((
1388                Node {
1389                    width: percent(100),
1390                    height: percent(100),
1391                    display: Display::Block,
1392                    ..default()
1393                },
1394                DespawnOnExit(super::Scene::Transformations),
1395            ))
1396            .with_children(|parent| {
1397                for (transformation, label, background) in [
1398                    (
1399                        UiTransform::from_rotation(Rot2::degrees(45.)),
1400                        "Rotate 45 degrees",
1401                        RED,
1402                    ),
1403                    (
1404                        UiTransform::from_scale(Vec2::new(2., 0.5)),
1405                        "Scale 2.x 0.5y",
1406                        GREEN,
1407                    ),
1408                    (
1409                        UiTransform::from_translation(Val2::px(-50., 50.)),
1410                        "Translate -50px x +50px y",
1411                        BLUE,
1412                    ),
1413                    (
1414                        UiTransform {
1415                            translation: Val2::px(50., 0.),
1416                            scale: Vec2::new(-1., 1.),
1417                            rotation: Rot2::degrees(30.),
1418                        },
1419                        "T 50px x\nS -1.x (refl)\nR 30deg",
1420                        DARK_CYAN,
1421                    ),
1422                ] {
1423                    parent
1424                        .spawn((Node {
1425                            width: percent(100),
1426                            margin: UiRect {
1427                                top: px(50),
1428                                bottom: px(50),
1429                                ..default()
1430                            },
1431                            align_items: AlignItems::Center,
1432                            justify_content: JustifyContent::SpaceAround,
1433                            ..default()
1434                        },))
1435                        .with_children(|row| {
1436                            row.spawn((
1437                                Text::new("Before Tf"),
1438                                Node {
1439                                    width: px(100),
1440                                    height: px(100),
1441                                    border_radius: BorderRadius::bottom_right(px(25.)),
1442                                    ..default()
1443                                },
1444                                BackgroundColor(background.into()),
1445                                TextFont::default(),
1446                            ));
1447                            row.spawn((
1448                                Text::new(label),
1449                                Node {
1450                                    width: px(100),
1451                                    height: px(100),
1452                                    border_radius: BorderRadius::bottom_right(px(25.)),
1453                                    ..default()
1454                                },
1455                                BackgroundColor(background.into()),
1456                                transformation,
1457                                TextFont::default(),
1458                            ));
1459                        });
1460                }
1461            });
1462    }
1463}
1464
1465#[cfg(feature = "bevy_ui_debug")]
1466mod debug_outlines {
1467    use bevy::{
1468        color::palettes::css::{BLUE, GRAY, RED},
1469        prelude::*,
1470        ui_render::UiDebugOptions,
1471    };
1472
1473    pub fn setup(mut commands: Commands, mut debug_options: ResMut<GlobalUiDebugOptions>) {
1474        debug_options.enabled = true;
1475        debug_options.line_width = 5.;
1476        debug_options.line_color_override = Some(LinearRgba::GREEN);
1477        debug_options.show_hidden = true;
1478        debug_options.show_clipped = true;
1479
1480        let debug_options: UiDebugOptions = (*debug_options.as_ref()).into();
1481
1482        commands.spawn((Camera2d, DespawnOnExit(super::Scene::DebugOutlines)));
1483        commands
1484            .spawn((
1485                Node {
1486                    width: percent(100),
1487                    height: percent(50),
1488                    align_items: AlignItems::Center,
1489                    justify_content: JustifyContent::SpaceAround,
1490                    ..default()
1491                },
1492                DespawnOnExit(super::Scene::DebugOutlines),
1493            ))
1494            .with_children(|parent| {
1495                parent.spawn((
1496                    Node {
1497                        width: px(100),
1498                        height: px(100),
1499                        ..default()
1500                    },
1501                    BackgroundColor(GRAY.into()),
1502                    UiTransform::from_rotation(Rot2::degrees(45.)),
1503                ));
1504
1505                parent.spawn((Text::new("Regular Text"), TextFont::default()));
1506
1507                parent.spawn((
1508                    Node {
1509                        width: px(100),
1510                        height: px(100),
1511                        ..default()
1512                    },
1513                    Text::new("Invisible"),
1514                    BackgroundColor(GRAY.into()),
1515                    TextFont::default(),
1516                    Visibility::Hidden,
1517                ));
1518
1519                parent
1520                    .spawn((
1521                        Node {
1522                            width: px(100),
1523                            height: px(100),
1524                            padding: UiRect {
1525                                left: px(25),
1526                                top: px(25),
1527                                ..Default::default()
1528                            },
1529                            overflow: Overflow::clip(),
1530                            ..default()
1531                        },
1532                        BackgroundColor(RED.into()),
1533                    ))
1534                    .with_children(|child| {
1535                        child.spawn((
1536                            Node {
1537                                min_width: px(100),
1538                                min_height: px(100),
1539                                ..default()
1540                            },
1541                            BackgroundColor(BLUE.into()),
1542                        ));
1543                    });
1544            });
1545
1546        commands
1547            .spawn((
1548                Node {
1549                    width: percent(100),
1550                    height: percent(50),
1551                    top: percent(50),
1552                    align_items: AlignItems::Center,
1553                    justify_content: JustifyContent::SpaceAround,
1554                    ..default()
1555                },
1556                DespawnOnExit(super::Scene::DebugOutlines),
1557            ))
1558            .with_children(|parent| {
1559                parent.spawn((
1560                    Node {
1561                        width: px(200),
1562                        height: px(200),
1563                        border: UiRect {
1564                            top: px(10),
1565                            bottom: px(20),
1566                            left: px(30),
1567                            right: px(40),
1568                        },
1569                        border_radius: BorderRadius::bottom_right(px(10)),
1570                        padding: UiRect {
1571                            top: px(40),
1572                            bottom: px(30),
1573                            left: px(20),
1574                            right: px(10),
1575                        },
1576                        ..default()
1577                    },
1578                    children![(
1579                        Text::new("border padding content outlines"),
1580                        TextFont::default(),
1581                        UiDebugOptions {
1582                            enabled: false,
1583                            ..default()
1584                        }
1585                    )],
1586                    UiDebugOptions {
1587                        outline_border_box: true,
1588                        outline_padding_box: true,
1589                        outline_content_box: true,
1590                        ignore_border_radius: false,
1591                        ..debug_options
1592                    },
1593                ));
1594
1595                // Vertical scrollbar (non-functional)
1596                parent.spawn((
1597                    Node {
1598                        flex_direction: FlexDirection::Column,
1599                        width: px(90),
1600                        height: px(230),
1601                        overflow: Overflow::scroll_y(),
1602                        scrollbar_width: 20.,
1603                        ..default()
1604                    },
1605                    ScrollPosition(Vec2::new(180., 180.)),
1606                    UiDebugOptions {
1607                        line_width: 3.,
1608                        outline_scrollbars: true,
1609                        show_hidden: false,
1610                        show_clipped: false,
1611                        ..debug_options
1612                    },
1613                    Children::spawn(SpawnIter((0..20).map(move |i| {
1614                        (
1615                            Node::default(),
1616                            children![(
1617                                Text(format!("Item {i}")),
1618                                UiDebugOptions {
1619                                    enabled: false,
1620                                    ..default()
1621                                }
1622                            )],
1623                            UiDebugOptions {
1624                                enabled: false,
1625                                ..default()
1626                            },
1627                        )
1628                    }))),
1629                ));
1630
1631                // Horizontal scrollbar (non-functional)
1632                parent.spawn((
1633                    Node {
1634                        flex_direction: FlexDirection::Row,
1635                        width: px(156),
1636                        height: px(70),
1637                        overflow: Overflow::scroll_x(),
1638                        scrollbar_width: 10.,
1639                        ..default()
1640                    },
1641                    UiDebugOptions {
1642                        line_width: 3.,
1643                        outline_scrollbars: true,
1644                        show_hidden: false,
1645                        show_clipped: false,
1646                        ..debug_options
1647                    },
1648                    Children::spawn(SpawnIter((0..20).map(move |i| {
1649                        (
1650                            Node::default(),
1651                            children![(
1652                                Text(format!("Item {i}")),
1653                                UiDebugOptions {
1654                                    enabled: false,
1655                                    ..default()
1656                                }
1657                            )],
1658                            UiDebugOptions {
1659                                enabled: false,
1660                                ..default()
1661                            },
1662                        )
1663                    }))),
1664                ));
1665
1666                // bi-directional scrollbar (non-functional)
1667                parent.spawn((
1668                    Node {
1669                        flex_direction: FlexDirection::Column,
1670                        width: px(230),
1671                        height: px(125),
1672                        overflow: Overflow::scroll(),
1673                        scrollbar_width: 20.,
1674                        ..default()
1675                    },
1676                    ScrollPosition(Vec2::new(300., 0.)),
1677                    UiDebugOptions {
1678                        line_width: 3.,
1679                        outline_scrollbars: true,
1680                        show_hidden: false,
1681                        show_clipped: false,
1682                        ..debug_options
1683                    },
1684                    Children::spawn(SpawnIter((0..6).map(move |i| {
1685                        (
1686                            Node {
1687                                flex_direction: FlexDirection::Row,
1688                                ..default()
1689                            },
1690                            Children::spawn(SpawnIter((0..6).map({
1691                                move |j| {
1692                                    (
1693                                        Text(format!("Item {}", (i * 5) + j)),
1694                                        UiDebugOptions {
1695                                            enabled: false,
1696                                            ..default()
1697                                        },
1698                                    )
1699                                }
1700                            }))),
1701                            UiDebugOptions {
1702                                enabled: false,
1703                                ..default()
1704                            },
1705                        )
1706                    }))),
1707                ));
1708            });
1709    }
```

examples/ui/scroll\_and\_overflow/overflow.rs ([line 36](../../src/overflow/overflow.rs.html#36))

```rust
13fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
14    commands.spawn(Camera2d);
15
16    let text_style = TextFont::default();
17
18    let image = asset_server.load("branding/icon.png");
19
20    commands
21        .spawn((
22            Node {
23                width: percent(100),
24                height: percent(100),
25                align_items: AlignItems::Center,
26                justify_content: JustifyContent::Center,
27                ..Default::default()
28            },
29            BackgroundColor(ANTIQUE_WHITE.into()),
30        ))
31        .with_children(|parent| {
32            for overflow in [
33                Overflow::visible(),
34                Overflow::clip_x(),
35                Overflow::clip_y(),
36                Overflow::clip(),
37            ] {
38                parent
39                    .spawn(Node {
40                        flex_direction: FlexDirection::Column,
41                        align_items: AlignItems::Center,
42                        margin: UiRect::horizontal(px(25)),
43                        ..Default::default()
44                    })
45                    .with_children(|parent| {
46                        let label = format!("{overflow:#?}");
47                        parent
48                            .spawn((
49                                Node {
50                                    padding: UiRect::all(px(10)),
51                                    margin: UiRect::bottom(px(25)),
52                                    ..Default::default()
53                                },
54                                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
55                            ))
56                            .with_children(|parent| {
57                                parent.spawn((Text::new(label), text_style.clone()));
58                            });
59                        parent
60                            .spawn((
61                                Node {
62                                    width: px(100),
63                                    height: px(100),
64                                    padding: UiRect {
65                                        left: px(25),
66                                        top: px(25),
67                                        ..Default::default()
68                                    },
69                                    border: UiRect::all(px(5)),
70                                    overflow,
71                                    ..default()
72                                },
73                                BorderColor::all(Color::BLACK),
74                                BackgroundColor(GRAY.into()),
75                            ))
76                            .with_children(|parent| {
77                                parent.spawn((
78                                    ImageNode::new(image.clone()),
79                                    Node {
80                                        min_width: px(100),
81                                        min_height: px(100),
82                                        ..default()
83                                    },
84                                    Interaction::default(),
85                                    Outline {
86                                        width: px(2),
87                                        offset: px(2),
88                                        color: Color::NONE,
89                                    },
90                                ));
91                            });
92                    });
93            }
94        });
95}
```

examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs ([line 63](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#63))

```rust
12fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
13    commands.spawn(Camera2d);
14
15    let image = asset_server.load("branding/icon.png");
16
17    commands
18        .spawn((
19            Node {
20                width: percent(100),
21                height: percent(100),
22                align_items: AlignItems::Center,
23                justify_content: JustifyContent::Center,
24                row_gap: px(40),
25                flex_direction: FlexDirection::Column,
26                ..default()
27            },
28            BackgroundColor(ANTIQUE_WHITE.into()),
29        ))
30        .with_children(|parent| {
31            for overflow_clip_margin in [
32                OverflowClipMargin::border_box().with_margin(25.),
33                OverflowClipMargin::border_box(),
34                OverflowClipMargin::padding_box(),
35                OverflowClipMargin::content_box(),
36            ] {
37                parent
38                    .spawn(Node {
39                        flex_direction: FlexDirection::Row,
40                        column_gap: px(20),
41                        ..default()
42                    })
43                    .with_children(|parent| {
44                        parent
45                            .spawn((
46                                Node {
47                                    padding: UiRect::all(px(10)),
48                                    margin: UiRect::bottom(px(25)),
49                                    ..default()
50                                },
51                                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
52                            ))
53                            .with_child(Text(format!("{overflow_clip_margin:#?}")));
54
55                        parent
56                            .spawn((
57                                Node {
58                                    margin: UiRect::top(px(10)),
59                                    width: px(100),
60                                    height: px(100),
61                                    padding: UiRect::all(px(20)),
62                                    border: UiRect::all(px(5)),
63                                    overflow: Overflow::clip(),
64                                    overflow_clip_margin,
65                                    ..default()
66                                },
67                                BackgroundColor(GRAY.into()),
68                                BorderColor::all(Color::BLACK),
69                            ))
70                            .with_children(|parent| {
71                                parent
72                                    .spawn((
73                                        Node {
74                                            min_width: px(50),
75                                            min_height: px(50),
76                                            ..default()
77                                        },
78                                        BackgroundColor(LIGHT_CYAN.into()),
79                                    ))
80                                    .with_child((
81                                        ImageNode::new(image.clone()),
82                                        Node {
83                                            min_width: px(100),
84                                            min_height: px(100),
85                                            ..default()
86                                        },
87                                    ));
88                            });
89                    });
90            }
91        });
92}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1272)

#### pub const fn [clip\_x](#method.clip_x)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Clip overflowing items on the x axis

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/overflow\_debug.rs ([line 249](../../src/overflow_debug/overflow_debug.rs.html#249))

```rust
235fn toggle_overflow(
236    mut containers: Query<&mut Node, With<Container>>,
237    instructions: Single<Entity, With<Instructions>>,
238    mut writer: TextUiWriter,
239) {
240    for mut node in &mut containers {
241        node.overflow = match node.overflow {
242            Overflow {
243                x: OverflowAxis::Visible,
244                y: OverflowAxis::Visible,
245            } => Overflow::clip_y(),
246            Overflow {
247                x: OverflowAxis::Visible,
248                y: OverflowAxis::Clip,
249            } => Overflow::clip_x(),
250            Overflow {
251                x: OverflowAxis::Clip,
252                y: OverflowAxis::Visible,
253            } => Overflow::clip(),
254            _ => Overflow::visible(),
255        };
256
257        let entity = *instructions;
258        *writer.text(entity, 1) = format!("{:?}", node.overflow);
259    }
260}
```

Hide additional examples

examples/testbed/ui.rs ([line 929](../../src/testbed_ui/ui.rs.html#929))

```rust
920    pub fn setup(mut commands: Commands) {
921        commands.spawn((Camera2d, DespawnOnExit(super::Scene::TextWrap)));
922
923        let root = commands
924            .spawn((
925                Node {
926                    flex_direction: FlexDirection::Column,
927                    width: px(200),
928                    height: percent(100),
929                    overflow: Overflow::clip_x(),
930                    ..default()
931                },
932                BackgroundColor(Color::BLACK),
933                DespawnOnExit(super::Scene::TextWrap),
934            ))
935            .id();
936
937        for linebreak in [
938            LineBreak::AnyCharacter,
939            LineBreak::WordBoundary,
940            LineBreak::WordOrCharacter,
941            LineBreak::NoWrap,
942        ] {
943            let messages = [
944                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string(),
945                "pneumonoultramicroscopicsilicovolcanoconiosis".to_string(),
946            ];
947
948            for (j, message) in messages.into_iter().enumerate() {
949                commands.entity(root).with_child((
950                    Text(message.clone()),
951                    TextLayout::new(Justify::Left, linebreak),
952                    BackgroundColor(Color::srgb(0.8 - j as f32 * 0.3, 0., 0.)),
953                ));
954            }
955        }
956    }
957}
958
959mod overflow {
960    use bevy::{color::palettes::css::*, prelude::*};
961
962    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
963        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Overflow)));
964        let image = asset_server.load("branding/icon.png");
965
966        commands
967            .spawn((
968                Node {
969                    width: percent(100),
970                    height: percent(100),
971                    align_items: AlignItems::Center,
972                    justify_content: JustifyContent::SpaceAround,
973                    ..Default::default()
974                },
975                BackgroundColor(BLUE.into()),
976                DespawnOnExit(super::Scene::Overflow),
977            ))
978            .with_children(|parent| {
979                for overflow in [
980                    Overflow::visible(),
981                    Overflow::clip_x(),
982                    Overflow::clip_y(),
983                    Overflow::clip(),
984                ] {
985                    parent
986                        .spawn((
987                            Node {
988                                width: px(100),
989                                height: px(100),
990                                padding: UiRect {
991                                    left: px(25),
992                                    top: px(25),
993                                    ..Default::default()
994                                },
995                                border: UiRect::all(px(5)),
996                                overflow,
997                                ..default()
998                            },
999                            BorderColor::all(RED),
1000                            BackgroundColor(Color::WHITE),
1001                        ))
1002                        .with_children(|parent| {
1003                            parent.spawn((
1004                                ImageNode::new(image.clone()),
1005                                Node {
1006                                    min_width: px(100),
1007                                    min_height: px(100),
1008                                    ..default()
1009                                },
1010                                Interaction::default(),
1011                                Outline {
1012                                    width: px(2),
1013                                    offset: px(2),
1014                                    color: Color::NONE,
1015                                },
1016                            ));
1017                        });
1018                }
1019            });
1020    }
```

examples/ui/text/text\_wrap\_debug.rs ([line 100](../../src/text_wrap_debug/text_wrap_debug.rs.html#100))

```rust
44fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
45    commands.spawn(Camera2d);
46
47    let text_font = TextFont {
48        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
49        font_size: FontSize::Px(12.0),
50        ..default()
51    };
52
53    let root = commands
54        .spawn((
55            Node {
56                width: percent(100),
57                height: percent(100),
58                flex_direction: FlexDirection::Column,
59                ..default()
60            },
61            BackgroundColor(Color::BLACK),
62        ))
63        .id();
64
65    for linebreak in [
66        LineBreak::AnyCharacter,
67        LineBreak::WordBoundary,
68        LineBreak::WordOrCharacter,
69        LineBreak::NoWrap,
70    ] {
71        let row_id = commands
72            .spawn(Node {
73                flex_direction: FlexDirection::Row,
74                justify_content: JustifyContent::SpaceAround,
75                align_items: AlignItems::Center,
76                width: percent(100),
77                height: percent(50),
78                ..default()
79            })
80            .id();
81
82        let justifications = vec![
83            JustifyContent::Center,
84            JustifyContent::FlexStart,
85            JustifyContent::FlexEnd,
86            JustifyContent::SpaceAround,
87            JustifyContent::SpaceBetween,
88            JustifyContent::SpaceEvenly,
89        ];
90
91        for (i, justification) in justifications.into_iter().enumerate() {
92            let c = 0.3 + i as f32 * 0.1;
93            let column_id = commands
94                .spawn((
95                    Node {
96                        justify_content: justification,
97                        flex_direction: FlexDirection::Column,
98                        width: percent(16),
99                        height: percent(95),
100                        overflow: Overflow::clip_x(),
101                        ..default()
102                    },
103                    BackgroundColor(Color::srgb(0.5, c, 1.0 - c)),
104                ))
105                .id();
106
107            let messages = [
108                format!("JustifyContent::{justification:?}"),
109                format!("LineBreakOn::{linebreak:?}"),
110                "Line 1\nLine 2".to_string(),
111                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas auctor, nunc ac faucibus fringilla.".to_string(),
112                "pneumonoultramicroscopicsilicovolcanoconiosis".to_string()
113            ];
114
115            for (j, message) in messages.into_iter().enumerate() {
116                commands.entity(column_id).with_child((
117                    Text(message.clone()),
118                    text_font.clone(),
119                    TextLayout::new(Justify::Left, linebreak),
120                    BackgroundColor(Color::srgb(0.8 - j as f32 * 0.2, 0., 0.)),
121                ));
122            }
123            commands.entity(row_id).add_child(column_id);
124        }
125        commands.entity(root).add_child(row_id);
126    }
127}
```

examples/ui/scroll\_and\_overflow/overflow.rs ([line 34](../../src/overflow/overflow.rs.html#34))

```rust
13fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
14    commands.spawn(Camera2d);
15
16    let text_style = TextFont::default();
17
18    let image = asset_server.load("branding/icon.png");
19
20    commands
21        .spawn((
22            Node {
23                width: percent(100),
24                height: percent(100),
25                align_items: AlignItems::Center,
26                justify_content: JustifyContent::Center,
27                ..Default::default()
28            },
29            BackgroundColor(ANTIQUE_WHITE.into()),
30        ))
31        .with_children(|parent| {
32            for overflow in [
33                Overflow::visible(),
34                Overflow::clip_x(),
35                Overflow::clip_y(),
36                Overflow::clip(),
37            ] {
38                parent
39                    .spawn(Node {
40                        flex_direction: FlexDirection::Column,
41                        align_items: AlignItems::Center,
42                        margin: UiRect::horizontal(px(25)),
43                        ..Default::default()
44                    })
45                    .with_children(|parent| {
46                        let label = format!("{overflow:#?}");
47                        parent
48                            .spawn((
49                                Node {
50                                    padding: UiRect::all(px(10)),
51                                    margin: UiRect::bottom(px(25)),
52                                    ..Default::default()
53                                },
54                                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
55                            ))
56                            .with_children(|parent| {
57                                parent.spawn((Text::new(label), text_style.clone()));
58                            });
59                        parent
60                            .spawn((
61                                Node {
62                                    width: px(100),
63                                    height: px(100),
64                                    padding: UiRect {
65                                        left: px(25),
66                                        top: px(25),
67                                        ..Default::default()
68                                    },
69                                    border: UiRect::all(px(5)),
70                                    overflow,
71                                    ..default()
72                                },
73                                BorderColor::all(Color::BLACK),
74                                BackgroundColor(GRAY.into()),
75                            ))
76                            .with_children(|parent| {
77                                parent.spawn((
78                                    ImageNode::new(image.clone()),
79                                    Node {
80                                        min_width: px(100),
81                                        min_height: px(100),
82                                        ..default()
83                                    },
84                                    Interaction::default(),
85                                    Outline {
86                                        width: px(2),
87                                        offset: px(2),
88                                        color: Color::NONE,
89                                    },
90                                ));
91                            });
92                    });
93            }
94        });
95}
```

examples/ui/text/multiple\_text\_inputs.rs ([line 118](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#118))

```rust
43fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
44    commands.spawn(Camera2d);
45
46    let font = TextFont {
47        font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
48        font_size: FontSize::Px(24.),
49        ..default()
50    };
51
52    commands
53        .spawn((
54            Node {
55                width: percent(100.),
56                height: percent(100.),
57                display: Display::Grid,
58                justify_content: JustifyContent::Center,
59                align_content: AlignContent::Center,
60                grid_template_columns: RepeatedGridTrack::px(3, 320.),
61                grid_template_rows: RepeatedGridTrack::auto(6),
62                row_gap: px(8.),
63                column_gap: px(8.),
64                ..default()
65            },
66            TabGroup::default(),
67        ))
68        .with_children(|parent| {
69            parent.spawn((
70                Text::new("Multiple Text Inputs Example"),
71                Node {
72                    grid_column: GridPlacement::span(3),
73                    justify_self: JustifySelf::Center,
74                    margin: px(16).bottom(),
75                    ..default()
76                },
77                TextColor::WHITE,
78                font.clone(),
79            ));
80
81            let label_font = font.clone().with_font_size(14.);
82            for label in ["EditableText", "value", "submission"] {
83                parent.spawn((
84                    Text::new(label),
85                    label_font.clone(),
86                    Node {
87                        justify_self: JustifySelf::Center,
88                        margin: px(-4).bottom(),
89                        ..default()
90                    },
91                ));
92            }
93
94            for row in 0..3 {
95                let mut input = parent.spawn((
96                    Node {
97                        border: px(4.).all(),
98                        padding: px(4.).all(),
99                        ..default()
100                    },
101                    EditableText::new(format!("Initial text {row}")),
102                    TextCursorStyle::default(),
103                    font.clone(),
104                    BackgroundColor(bevy::color::palettes::css::DARK_GREY.into()),
105                    TextInputRow(row),
106                    TextLayout::no_wrap(),
107                    TabIndex(row as i32),
108                    BorderColor::all(SLATE_300),
109                ));
110                if row == 0 {
111                    input.insert(AutoFocus);
112                }
113
114                parent.spawn((
115                    Node {
116                        border: px(4.).all(),
117                        padding: px(4.).all(),
118                        overflow: Overflow::clip_x(),
119                        overflow_clip_margin: OverflowClipMargin {
120                            visual_box: VisualBox::ContentBox,
121                            ..default()
122                        },
123                        ..default()
124                    },
125                    BackgroundColor(bevy::color::palettes::css::DARK_SLATE_BLUE.into()),
126                    BorderColor::all(Color::WHITE),
127                    children![(
128                        Text::default(),
129                        TextLayout::no_wrap(),
130                        font.clone(),
131                        BackgroundColor(bevy::color::palettes::css::DARK_SLATE_GRAY.into()),
132                        BorderColor::all(Color::WHITE),
133                        TextInputRow(row),
134                        TextOutput,
135                    )],
136                ));
137
138                parent.spawn((
139                    Node {
140                        border: px(4.).all(),
141                        padding: px(4.).all(),
142                        overflow: Overflow::clip_x(),
143                        overflow_clip_margin: OverflowClipMargin {
144                            visual_box: VisualBox::ContentBox,
145                            ..default()
146                        },
147
148                        ..default()
149                    },
150                    BackgroundColor(bevy::color::palettes::css::DARK_SLATE_BLUE.into()),
151                    BorderColor::all(Color::WHITE),
152                    children![(
153                        Text::default(),
154                        TextLayout::no_wrap(),
155                        font.clone(),
156                        TextInputRow(row),
157                        SubmitOutput,
158                    )],
159                ));
160            }
161
162            parent.spawn((
163                Text::new("Press Enter to submit"),
164                Node {
165                    grid_column: GridPlacement::span(3),
166                    justify_self: JustifySelf::Center,
167                    margin: px(16).top(),
168                    ..default()
169                },
170                font.clone(),
171            ));
172        });
173}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1280)

#### pub const fn [clip\_y](#method.clip_y)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Clip overflowing items on the y axis

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/overflow\_debug.rs ([line 245](../../src/overflow_debug/overflow_debug.rs.html#245))

```rust
235fn toggle_overflow(
236    mut containers: Query<&mut Node, With<Container>>,
237    instructions: Single<Entity, With<Instructions>>,
238    mut writer: TextUiWriter,
239) {
240    for mut node in &mut containers {
241        node.overflow = match node.overflow {
242            Overflow {
243                x: OverflowAxis::Visible,
244                y: OverflowAxis::Visible,
245            } => Overflow::clip_y(),
246            Overflow {
247                x: OverflowAxis::Visible,
248                y: OverflowAxis::Clip,
249            } => Overflow::clip_x(),
250            Overflow {
251                x: OverflowAxis::Clip,
252                y: OverflowAxis::Visible,
253            } => Overflow::clip(),
254            _ => Overflow::visible(),
255        };
256
257        let entity = *instructions;
258        *writer.text(entity, 1) = format!("{:?}", node.overflow);
259    }
260}
```

Hide additional examples

examples/testbed/ui.rs ([line 982](../../src/testbed_ui/ui.rs.html#982))

```rust
962    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
963        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Overflow)));
964        let image = asset_server.load("branding/icon.png");
965
966        commands
967            .spawn((
968                Node {
969                    width: percent(100),
970                    height: percent(100),
971                    align_items: AlignItems::Center,
972                    justify_content: JustifyContent::SpaceAround,
973                    ..Default::default()
974                },
975                BackgroundColor(BLUE.into()),
976                DespawnOnExit(super::Scene::Overflow),
977            ))
978            .with_children(|parent| {
979                for overflow in [
980                    Overflow::visible(),
981                    Overflow::clip_x(),
982                    Overflow::clip_y(),
983                    Overflow::clip(),
984                ] {
985                    parent
986                        .spawn((
987                            Node {
988                                width: px(100),
989                                height: px(100),
990                                padding: UiRect {
991                                    left: px(25),
992                                    top: px(25),
993                                    ..Default::default()
994                                },
995                                border: UiRect::all(px(5)),
996                                overflow,
997                                ..default()
998                            },
999                            BorderColor::all(RED),
1000                            BackgroundColor(Color::WHITE),
1001                        ))
1002                        .with_children(|parent| {
1003                            parent.spawn((
1004                                ImageNode::new(image.clone()),
1005                                Node {
1006                                    min_width: px(100),
1007                                    min_height: px(100),
1008                                    ..default()
1009                                },
1010                                Interaction::default(),
1011                                Outline {
1012                                    width: px(2),
1013                                    offset: px(2),
1014                                    color: Color::NONE,
1015                                },
1016                            ));
1017                        });
1018                }
1019            });
1020    }
```

examples/ui/scroll\_and\_overflow/overflow.rs ([line 35](../../src/overflow/overflow.rs.html#35))

```rust
13fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
14    commands.spawn(Camera2d);
15
16    let text_style = TextFont::default();
17
18    let image = asset_server.load("branding/icon.png");
19
20    commands
21        .spawn((
22            Node {
23                width: percent(100),
24                height: percent(100),
25                align_items: AlignItems::Center,
26                justify_content: JustifyContent::Center,
27                ..Default::default()
28            },
29            BackgroundColor(ANTIQUE_WHITE.into()),
30        ))
31        .with_children(|parent| {
32            for overflow in [
33                Overflow::visible(),
34                Overflow::clip_x(),
35                Overflow::clip_y(),
36                Overflow::clip(),
37            ] {
38                parent
39                    .spawn(Node {
40                        flex_direction: FlexDirection::Column,
41                        align_items: AlignItems::Center,
42                        margin: UiRect::horizontal(px(25)),
43                        ..Default::default()
44                    })
45                    .with_children(|parent| {
46                        let label = format!("{overflow:#?}");
47                        parent
48                            .spawn((
49                                Node {
50                                    padding: UiRect::all(px(10)),
51                                    margin: UiRect::bottom(px(25)),
52                                    ..Default::default()
53                                },
54                                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
55                            ))
56                            .with_children(|parent| {
57                                parent.spawn((Text::new(label), text_style.clone()));
58                            });
59                        parent
60                            .spawn((
61                                Node {
62                                    width: px(100),
63                                    height: px(100),
64                                    padding: UiRect {
65                                        left: px(25),
66                                        top: px(25),
67                                        ..Default::default()
68                                    },
69                                    border: UiRect::all(px(5)),
70                                    overflow,
71                                    ..default()
72                                },
73                                BorderColor::all(Color::BLACK),
74                                BackgroundColor(GRAY.into()),
75                            ))
76                            .with_children(|parent| {
77                                parent.spawn((
78                                    ImageNode::new(image.clone()),
79                                    Node {
80                                        min_width: px(100),
81                                        min_height: px(100),
82                                        ..default()
83                                    },
84                                    Interaction::default(),
85                                    Outline {
86                                        width: px(2),
87                                        offset: px(2),
88                                        color: Color::NONE,
89                                    },
90                                ));
91                            });
92                    });
93            }
94        });
95}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1288)

#### pub const fn [hidden](#method.hidden)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Hide overflowing items on both axes by influencing layout and then clipping

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1296)

#### pub const fn [hidden\_x](#method.hidden_x)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Hide overflowing items on the x axis by influencing layout and then clipping

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1304)

#### pub const fn [hidden\_y](#method.hidden_y)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Hide overflowing items on the y axis by influencing layout and then clipping

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1312)

#### pub const fn [is\_visible](#method.is_visible)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Overflow is visible on both axes

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1316)

#### pub const fn [scroll](#method.scroll)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/scroll.rs ([line 298](../../src/scroll/scroll.rs.html#298))

```rust
274fn bidirectional_scrolling_list(font_handle: Handle<Font>) -> impl Bundle {
275    (
276        Node {
277            flex_direction: FlexDirection::Column,
278            justify_content: JustifyContent::Center,
279            align_items: AlignItems::Center,
280            width: px(200),
281            ..default()
282        },
283        children![
284            (
285                Text::new("Bidirectionally Scrolling List"),
286                TextFont {
287                    font: font_handle.clone().into(),
288                    font_size: FONT_SIZE,
289                    ..default()
290                },
291                Label,
292            ),
293            (
294                Node {
295                    flex_direction: FlexDirection::Column,
296                    align_self: AlignSelf::Stretch,
297                    height: percent(50),
298                    overflow: Overflow::scroll(), // n.b.
299                    ..default()
300                },
301                BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
302                Children::spawn(SpawnIter((0..25).map(move |oi| {
303                    (
304                        Node {
305                            flex_direction: FlexDirection::Row,
306                            ..default()
307                        },
308                        Children::spawn(SpawnIter((0..10).map({
309                            let value = font_handle.clone();
310                            move |i| {
311                                (
312                                    Text(format!("Item {}", (oi * 10) + i)),
313                                    TextFont::from(value.clone()),
314                                    Label,
315                                    AccessibilityNode(Accessible::new(Role::ListItem)),
316                                )
317                            }
318                        }))),
319                    )
320                })))
321            )
322        ],
323    )
324}
325
326fn bidirectional_scrolling_list_with_sticky(font_handle: Handle<Font>) -> impl Bundle {
327    (
328        Node {
329            flex_direction: FlexDirection::Column,
330            justify_content: JustifyContent::Center,
331            align_items: AlignItems::Center,
332            width: px(200),
333            ..default()
334        },
335        children![
336            (
337                Text::new("Bidirectionally Scrolling List With Sticky Nodes"),
338                TextFont {
339                    font: font_handle.clone().into(),
340                    font_size: FONT_SIZE,
341                    ..default()
342                },
343                Label,
344            ),
345            (
346                Node {
347                    display: Display::Grid,
348                    align_self: AlignSelf::Stretch,
349                    height: percent(50),
350                    overflow: Overflow::scroll(), // n.b.
351                    grid_template_columns: RepeatedGridTrack::auto(30),
352                    ..default()
353                },
354                Children::spawn(SpawnIter(
355                    (0..30)
356                        .flat_map(|y| (0..30).map(move |x| (y, x)))
357                        .map(move |(y, x)| {
358                            let value = font_handle.clone();
359                            // Simple sticky nodes at top and left sides of UI node
360                            // can be achieved by combining such effects as
361                            // IgnoreScroll, ZIndex, BackgroundColor for child UI nodes.
362                            let ignore_scroll = BVec2 {
363                                x: x == 0,
364                                y: y == 0,
365                            };
366                            let (z_index, background_color, role) = match (x == 0, y == 0) {
367                                (true, true) => (2, RED, Role::RowHeader),
368                                (true, false) => (1, BLUE, Role::RowHeader),
369                                (false, true) => (1, BLUE, Role::ColumnHeader),
370                                (false, false) => (0, BLACK, Role::Cell),
371                            };
372                            (
373                                Text(format!("|{},{}|", y, x)),
374                                TextFont::from(value.clone()),
375                                TextLayout {
376                                    linebreak: LineBreak::NoWrap,
377                                    ..default()
378                                },
379                                Label,
380                                AccessibilityNode(Accessible::new(role)),
381                                IgnoreScroll(ignore_scroll),
382                                ZIndex(z_index),
383                                BackgroundColor(Color::Srgba(background_color)),
384                            )
385                        })
386                ))
387            )
388        ],
389    )
390}
391
392fn nested_scrolling_list(font_handle: Handle<Font>) -> impl Bundle {
393    (
394        Node {
395            flex_direction: FlexDirection::Column,
396            justify_content: JustifyContent::Center,
397            align_items: AlignItems::Center,
398            width: px(200),
399            ..default()
400        },
401        children![
402            (
403                // Title
404                Text::new("Nested Scrolling Lists"),
405                TextFont {
406                    font: font_handle.clone().into(),
407                    font_size: FONT_SIZE,
408                    ..default()
409                },
410                Label,
411            ),
412            (
413                // Outer, bi-directional scrolling container
414                Node {
415                    column_gap: px(20),
416                    flex_direction: FlexDirection::Row,
417                    align_self: AlignSelf::Stretch,
418                    height: percent(50),
419                    overflow: Overflow::scroll(),
420                    ..default()
421                },
422                BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
423                // Inner, scrolling columns
424                Children::spawn(SpawnIter((0..5).map(move |oi| {
425                    (
426                        Node {
427                            flex_direction: FlexDirection::Column,
428                            align_self: AlignSelf::Stretch,
429                            height: percent(200. / 5. * (oi as f32 + 1.)),
430                            overflow: Overflow::scroll_y(),
431                            ..default()
432                        },
433                        BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
434                        Children::spawn(SpawnIter((0..20).map({
435                            let value = font_handle.clone();
436                            move |i| {
437                                (
438                                    Text(format!("Item {}", (oi * 20) + i)),
439                                    TextFont::from(value.clone()),
440                                    Label,
441                                    AccessibilityNode(Accessible::new(Role::ListItem)),
442                                )
443                            }
444                        }))),
445                    )
446                })))
447            )
448        ],
449    )
450}
```

Hide additional examples

examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs ([line 34](../../src/drag_to_scroll/drag_to_scroll.rs.html#34))

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

examples/ui/scroll\_and\_overflow/scrollbars.rs ([line 74](../../src/scrollbars/scrollbars.rs.html#74))

```rust
51fn scroll_area_demo() -> impl Bundle {
52    (
53        // Frame element which contains the scroll area and scrollbars.
54        Node {
55            display: Display::Grid,
56            width: px(200),
57            height: px(150),
58            grid_template_columns: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
59            grid_template_rows: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
60            row_gap: px(2),
61            column_gap: px(2),
62            ..default()
63        },
64        Children::spawn((SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
65            // The actual scrolling area.
66            // Note that we're using `SpawnWith` here because we need to get the entity id of the
67            // scroll area in order to set the target of the scrollbars.
68            let scroll_area_id = parent
69                .spawn((
70                    Node {
71                        display: Display::Flex,
72                        flex_direction: FlexDirection::Column,
73                        padding: UiRect::all(px(4)),
74                        overflow: Overflow::scroll(),
75                        ..default()
76                    },
77                    BackgroundColor(colors::GRAY1.into()),
78                    ScrollPosition(Vec2::new(0.0, 10.0)),
79                    Children::spawn((
80                        // The actual content of the scrolling area
81                        Spawn(text_row("Alpha Wolf")),
82                        Spawn(text_row("Beta Blocker")),
83                        Spawn(text_row("Delta Sleep")),
84                        Spawn(text_row("Gamma Ray")),
85                        Spawn(text_row("Epsilon Eridani")),
86                        Spawn(text_row("Zeta Function")),
87                        Spawn(text_row("Lambda Calculus")),
88                        Spawn(text_row("Nu Metal")),
89                        Spawn(text_row("Pi Day")),
90                        Spawn(text_row("Chi Pants")),
91                        Spawn(text_row("Psi Powers")),
92                        Spawn(text_row("Omega Fatty Acid")),
93                    )),
94                ))
95                .id();
96
97            // Vertical scrollbar
98            parent.spawn((
99                Node {
100                    min_width: px(8),
101                    grid_row: GridPlacement::start(1),
102                    grid_column: GridPlacement::start(2),
103                    ..default()
104                },
105                Scrollbar {
106                    orientation: ControlOrientation::Vertical,
107                    target: scroll_area_id,
108                    min_thumb_length: 8.0,
109                },
110                Children::spawn(Spawn((
111                    Hovered::default(),
112                    BackgroundColor(colors::GRAY2.into()),
113                    BorderColor::all(colors::GRAY3),
114                    ScrollbarThumb {
115                        border_radius: BorderRadius::all(px(4)),
116                        border: px(1).all(),
117                    },
118                ))),
119            ));
120
121            // Horizontal scrollbar
122            parent.spawn((
123                Node {
124                    min_height: px(8),
125                    grid_row: GridPlacement::start(2),
126                    grid_column: GridPlacement::start(1),
127                    ..default()
128                },
129                Scrollbar {
130                    orientation: ControlOrientation::Horizontal,
131                    target: scroll_area_id,
132                    min_thumb_length: 8.0,
133                },
134                Children::spawn(Spawn((
135                    Hovered::default(),
136                    BackgroundColor(colors::GRAY2.into()),
137                    BorderColor::all(colors::GRAY3),
138                    ScrollbarThumb {
139                        border_radius: BorderRadius::all(px(4)),
140                        border: px(1).all(),
141                    },
142                ))),
143            ));
144        }),)),
145    )
146}
```

examples/testbed/ui.rs ([line 1672](../../src/testbed_ui/ui.rs.html#1672))

```rust
1473    pub fn setup(mut commands: Commands, mut debug_options: ResMut<GlobalUiDebugOptions>) {
1474        debug_options.enabled = true;
1475        debug_options.line_width = 5.;
1476        debug_options.line_color_override = Some(LinearRgba::GREEN);
1477        debug_options.show_hidden = true;
1478        debug_options.show_clipped = true;
1479
1480        let debug_options: UiDebugOptions = (*debug_options.as_ref()).into();
1481
1482        commands.spawn((Camera2d, DespawnOnExit(super::Scene::DebugOutlines)));
1483        commands
1484            .spawn((
1485                Node {
1486                    width: percent(100),
1487                    height: percent(50),
1488                    align_items: AlignItems::Center,
1489                    justify_content: JustifyContent::SpaceAround,
1490                    ..default()
1491                },
1492                DespawnOnExit(super::Scene::DebugOutlines),
1493            ))
1494            .with_children(|parent| {
1495                parent.spawn((
1496                    Node {
1497                        width: px(100),
1498                        height: px(100),
1499                        ..default()
1500                    },
1501                    BackgroundColor(GRAY.into()),
1502                    UiTransform::from_rotation(Rot2::degrees(45.)),
1503                ));
1504
1505                parent.spawn((Text::new("Regular Text"), TextFont::default()));
1506
1507                parent.spawn((
1508                    Node {
1509                        width: px(100),
1510                        height: px(100),
1511                        ..default()
1512                    },
1513                    Text::new("Invisible"),
1514                    BackgroundColor(GRAY.into()),
1515                    TextFont::default(),
1516                    Visibility::Hidden,
1517                ));
1518
1519                parent
1520                    .spawn((
1521                        Node {
1522                            width: px(100),
1523                            height: px(100),
1524                            padding: UiRect {
1525                                left: px(25),
1526                                top: px(25),
1527                                ..Default::default()
1528                            },
1529                            overflow: Overflow::clip(),
1530                            ..default()
1531                        },
1532                        BackgroundColor(RED.into()),
1533                    ))
1534                    .with_children(|child| {
1535                        child.spawn((
1536                            Node {
1537                                min_width: px(100),
1538                                min_height: px(100),
1539                                ..default()
1540                            },
1541                            BackgroundColor(BLUE.into()),
1542                        ));
1543                    });
1544            });
1545
1546        commands
1547            .spawn((
1548                Node {
1549                    width: percent(100),
1550                    height: percent(50),
1551                    top: percent(50),
1552                    align_items: AlignItems::Center,
1553                    justify_content: JustifyContent::SpaceAround,
1554                    ..default()
1555                },
1556                DespawnOnExit(super::Scene::DebugOutlines),
1557            ))
1558            .with_children(|parent| {
1559                parent.spawn((
1560                    Node {
1561                        width: px(200),
1562                        height: px(200),
1563                        border: UiRect {
1564                            top: px(10),
1565                            bottom: px(20),
1566                            left: px(30),
1567                            right: px(40),
1568                        },
1569                        border_radius: BorderRadius::bottom_right(px(10)),
1570                        padding: UiRect {
1571                            top: px(40),
1572                            bottom: px(30),
1573                            left: px(20),
1574                            right: px(10),
1575                        },
1576                        ..default()
1577                    },
1578                    children![(
1579                        Text::new("border padding content outlines"),
1580                        TextFont::default(),
1581                        UiDebugOptions {
1582                            enabled: false,
1583                            ..default()
1584                        }
1585                    )],
1586                    UiDebugOptions {
1587                        outline_border_box: true,
1588                        outline_padding_box: true,
1589                        outline_content_box: true,
1590                        ignore_border_radius: false,
1591                        ..debug_options
1592                    },
1593                ));
1594
1595                // Vertical scrollbar (non-functional)
1596                parent.spawn((
1597                    Node {
1598                        flex_direction: FlexDirection::Column,
1599                        width: px(90),
1600                        height: px(230),
1601                        overflow: Overflow::scroll_y(),
1602                        scrollbar_width: 20.,
1603                        ..default()
1604                    },
1605                    ScrollPosition(Vec2::new(180., 180.)),
1606                    UiDebugOptions {
1607                        line_width: 3.,
1608                        outline_scrollbars: true,
1609                        show_hidden: false,
1610                        show_clipped: false,
1611                        ..debug_options
1612                    },
1613                    Children::spawn(SpawnIter((0..20).map(move |i| {
1614                        (
1615                            Node::default(),
1616                            children![(
1617                                Text(format!("Item {i}")),
1618                                UiDebugOptions {
1619                                    enabled: false,
1620                                    ..default()
1621                                }
1622                            )],
1623                            UiDebugOptions {
1624                                enabled: false,
1625                                ..default()
1626                            },
1627                        )
1628                    }))),
1629                ));
1630
1631                // Horizontal scrollbar (non-functional)
1632                parent.spawn((
1633                    Node {
1634                        flex_direction: FlexDirection::Row,
1635                        width: px(156),
1636                        height: px(70),
1637                        overflow: Overflow::scroll_x(),
1638                        scrollbar_width: 10.,
1639                        ..default()
1640                    },
1641                    UiDebugOptions {
1642                        line_width: 3.,
1643                        outline_scrollbars: true,
1644                        show_hidden: false,
1645                        show_clipped: false,
1646                        ..debug_options
1647                    },
1648                    Children::spawn(SpawnIter((0..20).map(move |i| {
1649                        (
1650                            Node::default(),
1651                            children![(
1652                                Text(format!("Item {i}")),
1653                                UiDebugOptions {
1654                                    enabled: false,
1655                                    ..default()
1656                                }
1657                            )],
1658                            UiDebugOptions {
1659                                enabled: false,
1660                                ..default()
1661                            },
1662                        )
1663                    }))),
1664                ));
1665
1666                // bi-directional scrollbar (non-functional)
1667                parent.spawn((
1668                    Node {
1669                        flex_direction: FlexDirection::Column,
1670                        width: px(230),
1671                        height: px(125),
1672                        overflow: Overflow::scroll(),
1673                        scrollbar_width: 20.,
1674                        ..default()
1675                    },
1676                    ScrollPosition(Vec2::new(300., 0.)),
1677                    UiDebugOptions {
1678                        line_width: 3.,
1679                        outline_scrollbars: true,
1680                        show_hidden: false,
1681                        show_clipped: false,
1682                        ..debug_options
1683                    },
1684                    Children::spawn(SpawnIter((0..6).map(move |i| {
1685                        (
1686                            Node {
1687                                flex_direction: FlexDirection::Row,
1688                                ..default()
1689                            },
1690                            Children::spawn(SpawnIter((0..6).map({
1691                                move |j| {
1692                                    (
1693                                        Text(format!("Item {}", (i * 5) + j)),
1694                                        UiDebugOptions {
1695                                            enabled: false,
1696                                            ..default()
1697                                        },
1698                                    )
1699                                }
1700                            }))),
1701                            UiDebugOptions {
1702                                enabled: false,
1703                                ..default()
1704                            },
1705                        )
1706                    }))),
1707                ));
1708            });
1709    }
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1324)

#### pub const fn [scroll\_x](#method.scroll_x)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Scroll overflowing items on the x axis

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/scroll.rs ([line 153](../../src/scroll/scroll.rs.html#153))

```rust
110fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
111    // Camera
112    commands.spawn((Camera2d, IsDefaultUiCamera));
113
114    // Font
115    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
116
117    // root node
118    commands
119        .spawn(Node {
120            width: percent(100),
121            height: percent(100),
122            justify_content: JustifyContent::SpaceBetween,
123            flex_direction: FlexDirection::Column,
124            ..default()
125        })
126        .with_children(|parent| {
127            // horizontal scroll example
128            parent
129                .spawn(Node {
130                    width: percent(100),
131                    flex_direction: FlexDirection::Column,
132                    ..default()
133                })
134                .with_children(|parent| {
135                    // header
136                    parent.spawn((
137                        Text::new("Horizontally Scrolling list (Ctrl + MouseWheel)"),
138                        TextFont {
139                            font: font_handle.clone().into(),
140                            font_size: FONT_SIZE,
141                            ..default()
142                        },
143                        Label,
144                    ));
145
146                    // horizontal scroll container
147                    parent
148                        .spawn((
149                            Node {
150                                width: percent(80),
151                                margin: UiRect::all(px(10)),
152                                flex_direction: FlexDirection::Row,
153                                overflow: Overflow::scroll_x(), // n.b.
154                                ..default()
155                            },
156                            BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
157                        ))
158                        .with_children(|parent| {
159                            for i in 0..100 {
160                                parent
161                                    .spawn((
162                                        Text(format!("Item {i}")),
163                                        TextFont {
164                                            font: font_handle.clone().into(),
165                                            ..default()
166                                        },
167                                        Label,
168                                        AccessibilityNode(Accessible::new(Role::ListItem)),
169                                        Node {
170                                            min_width: px(200),
171                                            align_content: AlignContent::Center,
172                                            ..default()
173                                        },
174                                    ))
175                                    .observe(
176                                        |press: On<Pointer<Press>>, mut commands: Commands| {
177                                            if press.event().button == PointerButton::Primary {
178                                                commands.entity(press.entity).despawn();
179                                            }
180                                        },
181                                    );
182                            }
183                        });
184                });
185
186            // container for all other examples
187            parent.spawn((
188                Node {
189                    width: percent(100),
190                    height: percent(100),
191                    flex_direction: FlexDirection::Row,
192                    justify_content: JustifyContent::SpaceBetween,
193                    ..default()
194                },
195                children![
196                    vertically_scrolling_list(asset_server.load("fonts/FiraSans-Bold.ttf")),
197                    bidirectional_scrolling_list(asset_server.load("fonts/FiraSans-Bold.ttf")),
198                    bidirectional_scrolling_list_with_sticky(
199                        asset_server.load("fonts/FiraSans-Bold.ttf")
200                    ),
201                    nested_scrolling_list(asset_server.load("fonts/FiraSans-Bold.ttf")),
202                ],
203            ));
204        });
205}
```

Hide additional examples

examples/testbed/ui.rs ([line 1637](../../src/testbed_ui/ui.rs.html#1637))

```rust
1473    pub fn setup(mut commands: Commands, mut debug_options: ResMut<GlobalUiDebugOptions>) {
1474        debug_options.enabled = true;
1475        debug_options.line_width = 5.;
1476        debug_options.line_color_override = Some(LinearRgba::GREEN);
1477        debug_options.show_hidden = true;
1478        debug_options.show_clipped = true;
1479
1480        let debug_options: UiDebugOptions = (*debug_options.as_ref()).into();
1481
1482        commands.spawn((Camera2d, DespawnOnExit(super::Scene::DebugOutlines)));
1483        commands
1484            .spawn((
1485                Node {
1486                    width: percent(100),
1487                    height: percent(50),
1488                    align_items: AlignItems::Center,
1489                    justify_content: JustifyContent::SpaceAround,
1490                    ..default()
1491                },
1492                DespawnOnExit(super::Scene::DebugOutlines),
1493            ))
1494            .with_children(|parent| {
1495                parent.spawn((
1496                    Node {
1497                        width: px(100),
1498                        height: px(100),
1499                        ..default()
1500                    },
1501                    BackgroundColor(GRAY.into()),
1502                    UiTransform::from_rotation(Rot2::degrees(45.)),
1503                ));
1504
1505                parent.spawn((Text::new("Regular Text"), TextFont::default()));
1506
1507                parent.spawn((
1508                    Node {
1509                        width: px(100),
1510                        height: px(100),
1511                        ..default()
1512                    },
1513                    Text::new("Invisible"),
1514                    BackgroundColor(GRAY.into()),
1515                    TextFont::default(),
1516                    Visibility::Hidden,
1517                ));
1518
1519                parent
1520                    .spawn((
1521                        Node {
1522                            width: px(100),
1523                            height: px(100),
1524                            padding: UiRect {
1525                                left: px(25),
1526                                top: px(25),
1527                                ..Default::default()
1528                            },
1529                            overflow: Overflow::clip(),
1530                            ..default()
1531                        },
1532                        BackgroundColor(RED.into()),
1533                    ))
1534                    .with_children(|child| {
1535                        child.spawn((
1536                            Node {
1537                                min_width: px(100),
1538                                min_height: px(100),
1539                                ..default()
1540                            },
1541                            BackgroundColor(BLUE.into()),
1542                        ));
1543                    });
1544            });
1545
1546        commands
1547            .spawn((
1548                Node {
1549                    width: percent(100),
1550                    height: percent(50),
1551                    top: percent(50),
1552                    align_items: AlignItems::Center,
1553                    justify_content: JustifyContent::SpaceAround,
1554                    ..default()
1555                },
1556                DespawnOnExit(super::Scene::DebugOutlines),
1557            ))
1558            .with_children(|parent| {
1559                parent.spawn((
1560                    Node {
1561                        width: px(200),
1562                        height: px(200),
1563                        border: UiRect {
1564                            top: px(10),
1565                            bottom: px(20),
1566                            left: px(30),
1567                            right: px(40),
1568                        },
1569                        border_radius: BorderRadius::bottom_right(px(10)),
1570                        padding: UiRect {
1571                            top: px(40),
1572                            bottom: px(30),
1573                            left: px(20),
1574                            right: px(10),
1575                        },
1576                        ..default()
1577                    },
1578                    children![(
1579                        Text::new("border padding content outlines"),
1580                        TextFont::default(),
1581                        UiDebugOptions {
1582                            enabled: false,
1583                            ..default()
1584                        }
1585                    )],
1586                    UiDebugOptions {
1587                        outline_border_box: true,
1588                        outline_padding_box: true,
1589                        outline_content_box: true,
1590                        ignore_border_radius: false,
1591                        ..debug_options
1592                    },
1593                ));
1594
1595                // Vertical scrollbar (non-functional)
1596                parent.spawn((
1597                    Node {
1598                        flex_direction: FlexDirection::Column,
1599                        width: px(90),
1600                        height: px(230),
1601                        overflow: Overflow::scroll_y(),
1602                        scrollbar_width: 20.,
1603                        ..default()
1604                    },
1605                    ScrollPosition(Vec2::new(180., 180.)),
1606                    UiDebugOptions {
1607                        line_width: 3.,
1608                        outline_scrollbars: true,
1609                        show_hidden: false,
1610                        show_clipped: false,
1611                        ..debug_options
1612                    },
1613                    Children::spawn(SpawnIter((0..20).map(move |i| {
1614                        (
1615                            Node::default(),
1616                            children![(
1617                                Text(format!("Item {i}")),
1618                                UiDebugOptions {
1619                                    enabled: false,
1620                                    ..default()
1621                                }
1622                            )],
1623                            UiDebugOptions {
1624                                enabled: false,
1625                                ..default()
1626                            },
1627                        )
1628                    }))),
1629                ));
1630
1631                // Horizontal scrollbar (non-functional)
1632                parent.spawn((
1633                    Node {
1634                        flex_direction: FlexDirection::Row,
1635                        width: px(156),
1636                        height: px(70),
1637                        overflow: Overflow::scroll_x(),
1638                        scrollbar_width: 10.,
1639                        ..default()
1640                    },
1641                    UiDebugOptions {
1642                        line_width: 3.,
1643                        outline_scrollbars: true,
1644                        show_hidden: false,
1645                        show_clipped: false,
1646                        ..debug_options
1647                    },
1648                    Children::spawn(SpawnIter((0..20).map(move |i| {
1649                        (
1650                            Node::default(),
1651                            children![(
1652                                Text(format!("Item {i}")),
1653                                UiDebugOptions {
1654                                    enabled: false,
1655                                    ..default()
1656                                }
1657                            )],
1658                            UiDebugOptions {
1659                                enabled: false,
1660                                ..default()
1661                            },
1662                        )
1663                    }))),
1664                ));
1665
1666                // bi-directional scrollbar (non-functional)
1667                parent.spawn((
1668                    Node {
1669                        flex_direction: FlexDirection::Column,
1670                        width: px(230),
1671                        height: px(125),
1672                        overflow: Overflow::scroll(),
1673                        scrollbar_width: 20.,
1674                        ..default()
1675                    },
1676                    ScrollPosition(Vec2::new(300., 0.)),
1677                    UiDebugOptions {
1678                        line_width: 3.,
1679                        outline_scrollbars: true,
1680                        show_hidden: false,
1681                        show_clipped: false,
1682                        ..debug_options
1683                    },
1684                    Children::spawn(SpawnIter((0..6).map(move |i| {
1685                        (
1686                            Node {
1687                                flex_direction: FlexDirection::Row,
1688                                ..default()
1689                            },
1690                            Children::spawn(SpawnIter((0..6).map({
1691                                move |j| {
1692                                    (
1693                                        Text(format!("Item {}", (i * 5) + j)),
1694                                        UiDebugOptions {
1695                                            enabled: false,
1696                                            ..default()
1697                                        },
1698                                    )
1699                                }
1700                            }))),
1701                            UiDebugOptions {
1702                                enabled: false,
1703                                ..default()
1704                            },
1705                        )
1706                    }))),
1707                ));
1708            });
1709    }
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1332)

#### pub const fn [scroll\_y](#method.scroll_y)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Scroll overflowing items on the y axis

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/scroll.rs ([line 233](../../src/scroll/scroll.rs.html#233))

```rust
207fn vertically_scrolling_list(font_handle: Handle<Font>) -> impl Bundle {
208    (
209        Node {
210            flex_direction: FlexDirection::Column,
211            justify_content: JustifyContent::Center,
212            align_items: AlignItems::Center,
213            width: px(200),
214            ..default()
215        },
216        children![
217            (
218                // Title
219                Text::new("Vertically Scrolling List"),
220                TextFont {
221                    font: font_handle.clone().into(),
222                    font_size: FONT_SIZE,
223                    ..default()
224                },
225                Label,
226            ),
227            (
228                // Scrolling list
229                Node {
230                    flex_direction: FlexDirection::Column,
231                    align_self: AlignSelf::Stretch,
232                    height: percent(50),
233                    overflow: Overflow::scroll_y(), // n.b.
234                    scrollbar_width: 20.,
235                    ..default()
236                },
237                #[cfg(feature = "bevy_ui_debug")]
238                UiDebugOptions {
239                    enabled: true,
240                    outline_border_box: false,
241                    outline_padding_box: false,
242                    outline_content_box: false,
243                    outline_scrollbars: true,
244                    line_width: 2.,
245                    line_color_override: None,
246                    show_hidden: false,
247                    show_clipped: true,
248                    ignore_border_radius: true,
249                },
250                BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
251                Children::spawn(SpawnIter((0..25).map(move |i| {
252                    (
253                        Node {
254                            min_height: px(LINE_HEIGHT),
255                            max_height: px(LINE_HEIGHT),
256                            ..default()
257                        },
258                        children![(
259                            Text(format!("Item {i}")),
260                            TextFont {
261                                font: font_handle.clone().into(),
262                                ..default()
263                            },
264                            Label,
265                            AccessibilityNode(Accessible::new(Role::ListItem)),
266                        )],
267                    )
268                })))
269            ),
270        ],
271    )
272}
273
274fn bidirectional_scrolling_list(font_handle: Handle<Font>) -> impl Bundle {
275    (
276        Node {
277            flex_direction: FlexDirection::Column,
278            justify_content: JustifyContent::Center,
279            align_items: AlignItems::Center,
280            width: px(200),
281            ..default()
282        },
283        children![
284            (
285                Text::new("Bidirectionally Scrolling List"),
286                TextFont {
287                    font: font_handle.clone().into(),
288                    font_size: FONT_SIZE,
289                    ..default()
290                },
291                Label,
292            ),
293            (
294                Node {
295                    flex_direction: FlexDirection::Column,
296                    align_self: AlignSelf::Stretch,
297                    height: percent(50),
298                    overflow: Overflow::scroll(), // n.b.
299                    ..default()
300                },
301                BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
302                Children::spawn(SpawnIter((0..25).map(move |oi| {
303                    (
304                        Node {
305                            flex_direction: FlexDirection::Row,
306                            ..default()
307                        },
308                        Children::spawn(SpawnIter((0..10).map({
309                            let value = font_handle.clone();
310                            move |i| {
311                                (
312                                    Text(format!("Item {}", (oi * 10) + i)),
313                                    TextFont::from(value.clone()),
314                                    Label,
315                                    AccessibilityNode(Accessible::new(Role::ListItem)),
316                                )
317                            }
318                        }))),
319                    )
320                })))
321            )
322        ],
323    )
324}
325
326fn bidirectional_scrolling_list_with_sticky(font_handle: Handle<Font>) -> impl Bundle {
327    (
328        Node {
329            flex_direction: FlexDirection::Column,
330            justify_content: JustifyContent::Center,
331            align_items: AlignItems::Center,
332            width: px(200),
333            ..default()
334        },
335        children![
336            (
337                Text::new("Bidirectionally Scrolling List With Sticky Nodes"),
338                TextFont {
339                    font: font_handle.clone().into(),
340                    font_size: FONT_SIZE,
341                    ..default()
342                },
343                Label,
344            ),
345            (
346                Node {
347                    display: Display::Grid,
348                    align_self: AlignSelf::Stretch,
349                    height: percent(50),
350                    overflow: Overflow::scroll(), // n.b.
351                    grid_template_columns: RepeatedGridTrack::auto(30),
352                    ..default()
353                },
354                Children::spawn(SpawnIter(
355                    (0..30)
356                        .flat_map(|y| (0..30).map(move |x| (y, x)))
357                        .map(move |(y, x)| {
358                            let value = font_handle.clone();
359                            // Simple sticky nodes at top and left sides of UI node
360                            // can be achieved by combining such effects as
361                            // IgnoreScroll, ZIndex, BackgroundColor for child UI nodes.
362                            let ignore_scroll = BVec2 {
363                                x: x == 0,
364                                y: y == 0,
365                            };
366                            let (z_index, background_color, role) = match (x == 0, y == 0) {
367                                (true, true) => (2, RED, Role::RowHeader),
368                                (true, false) => (1, BLUE, Role::RowHeader),
369                                (false, true) => (1, BLUE, Role::ColumnHeader),
370                                (false, false) => (0, BLACK, Role::Cell),
371                            };
372                            (
373                                Text(format!("|{},{}|", y, x)),
374                                TextFont::from(value.clone()),
375                                TextLayout {
376                                    linebreak: LineBreak::NoWrap,
377                                    ..default()
378                                },
379                                Label,
380                                AccessibilityNode(Accessible::new(role)),
381                                IgnoreScroll(ignore_scroll),
382                                ZIndex(z_index),
383                                BackgroundColor(Color::Srgba(background_color)),
384                            )
385                        })
386                ))
387            )
388        ],
389    )
390}
391
392fn nested_scrolling_list(font_handle: Handle<Font>) -> impl Bundle {
393    (
394        Node {
395            flex_direction: FlexDirection::Column,
396            justify_content: JustifyContent::Center,
397            align_items: AlignItems::Center,
398            width: px(200),
399            ..default()
400        },
401        children![
402            (
403                // Title
404                Text::new("Nested Scrolling Lists"),
405                TextFont {
406                    font: font_handle.clone().into(),
407                    font_size: FONT_SIZE,
408                    ..default()
409                },
410                Label,
411            ),
412            (
413                // Outer, bi-directional scrolling container
414                Node {
415                    column_gap: px(20),
416                    flex_direction: FlexDirection::Row,
417                    align_self: AlignSelf::Stretch,
418                    height: percent(50),
419                    overflow: Overflow::scroll(),
420                    ..default()
421                },
422                BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
423                // Inner, scrolling columns
424                Children::spawn(SpawnIter((0..5).map(move |oi| {
425                    (
426                        Node {
427                            flex_direction: FlexDirection::Column,
428                            align_self: AlignSelf::Stretch,
429                            height: percent(200. / 5. * (oi as f32 + 1.)),
430                            overflow: Overflow::scroll_y(),
431                            ..default()
432                        },
433                        BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
434                        Children::spawn(SpawnIter((0..20).map({
435                            let value = font_handle.clone();
436                            move |i| {
437                                (
438                                    Text(format!("Item {}", (oi * 20) + i)),
439                                    TextFont::from(value.clone()),
440                                    Label,
441                                    AccessibilityNode(Accessible::new(Role::ListItem)),
442                                )
443                            }
444                        }))),
445                    )
446                })))
447            )
448        ],
449    )
450}
```

Hide additional examples

examples/ui/text/system\_fonts.rs ([line 51](../../src/system_fonts/system_fonts.rs.html#51))

```rust
16fn setup(mut commands: Commands, mut font_system: ResMut<FontCx>) {
17    let mut families: Vec<String> = font_system
18        .context
19        .collection
20        .family_names()
21        .map(ToOwned::to_owned)
22        .collect();
23    families.sort_unstable();
24    families.dedup();
25    let family_count = families.len();
26
27    commands.spawn(Camera2d);
28
29    commands
30        .spawn((
31            Node {
32                flex_direction: FlexDirection::Column,
33                width: percent(100),
34                height: percent(100),
35                align_items: AlignItems::Center,
36                row_gap: px(10.),
37                ..default()
38            },
39            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
40        ))
41        .with_children(move |builder| {
42            builder.spawn(Text::new(format!(
43                "Total available fonts: {}",
44                family_count,
45            )));
46
47            builder
48                .spawn(Node {
49                    flex_direction: FlexDirection::Column,
50                    row_gap: px(6),
51                    overflow: Overflow::scroll_y(),
52                    align_items: AlignItems::Stretch,
53                    ..default()
54                })
55                .with_children(|builder| {
56                    for family in families {
57                        let font = FontSource::Family(family.clone().into());
58                        builder.spawn((
59                            Node {
60                                display: Display::Grid,
61                                grid_template_columns: vec![
62                                    GridTrack::flex(1.),
63                                    GridTrack::flex(1.),
64                                ],
65                                padding: px(6).all(),
66                                column_gap: px(50.),
67                                ..default()
68                            },
69                            BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
70                            children![
71                                (
72                                    Text::new(&family),
73                                    TextFont { font, ..default() },
74                                    TextLayout::no_wrap()
75                                ),
76                                (Text::new(family), TextLayout::no_wrap()),
77                            ],
78                        ));
79                    }
80                })
81                .observe(
82                    |on_scroll: On<Pointer<Scroll>>,
83                     mut query: Query<(&mut ScrollPosition, &ComputedNode)>| {
84                        if let Ok((mut scroll_position, node)) = query.get_mut(on_scroll.entity) {
85                            let dy = match on_scroll.unit {
86                                MouseScrollUnit::Line => on_scroll.y * 20.,
87                                MouseScrollUnit::Pixel => on_scroll.y,
88                            };
89                            let range = (node.content_size.y - node.size.y).max(0.)
90                                * node.inverse_scale_factor;
91                            scroll_position.y = (scroll_position.y - dy).clamp(0., range);
92                        }
93                    },
94                );
95        });
96}
```

examples/testbed/ui.rs ([line 1601](../../src/testbed_ui/ui.rs.html#1601))

```rust
1473    pub fn setup(mut commands: Commands, mut debug_options: ResMut<GlobalUiDebugOptions>) {
1474        debug_options.enabled = true;
1475        debug_options.line_width = 5.;
1476        debug_options.line_color_override = Some(LinearRgba::GREEN);
1477        debug_options.show_hidden = true;
1478        debug_options.show_clipped = true;
1479
1480        let debug_options: UiDebugOptions = (*debug_options.as_ref()).into();
1481
1482        commands.spawn((Camera2d, DespawnOnExit(super::Scene::DebugOutlines)));
1483        commands
1484            .spawn((
1485                Node {
1486                    width: percent(100),
1487                    height: percent(50),
1488                    align_items: AlignItems::Center,
1489                    justify_content: JustifyContent::SpaceAround,
1490                    ..default()
1491                },
1492                DespawnOnExit(super::Scene::DebugOutlines),
1493            ))
1494            .with_children(|parent| {
1495                parent.spawn((
1496                    Node {
1497                        width: px(100),
1498                        height: px(100),
1499                        ..default()
1500                    },
1501                    BackgroundColor(GRAY.into()),
1502                    UiTransform::from_rotation(Rot2::degrees(45.)),
1503                ));
1504
1505                parent.spawn((Text::new("Regular Text"), TextFont::default()));
1506
1507                parent.spawn((
1508                    Node {
1509                        width: px(100),
1510                        height: px(100),
1511                        ..default()
1512                    },
1513                    Text::new("Invisible"),
1514                    BackgroundColor(GRAY.into()),
1515                    TextFont::default(),
1516                    Visibility::Hidden,
1517                ));
1518
1519                parent
1520                    .spawn((
1521                        Node {
1522                            width: px(100),
1523                            height: px(100),
1524                            padding: UiRect {
1525                                left: px(25),
1526                                top: px(25),
1527                                ..Default::default()
1528                            },
1529                            overflow: Overflow::clip(),
1530                            ..default()
1531                        },
1532                        BackgroundColor(RED.into()),
1533                    ))
1534                    .with_children(|child| {
1535                        child.spawn((
1536                            Node {
1537                                min_width: px(100),
1538                                min_height: px(100),
1539                                ..default()
1540                            },
1541                            BackgroundColor(BLUE.into()),
1542                        ));
1543                    });
1544            });
1545
1546        commands
1547            .spawn((
1548                Node {
1549                    width: percent(100),
1550                    height: percent(50),
1551                    top: percent(50),
1552                    align_items: AlignItems::Center,
1553                    justify_content: JustifyContent::SpaceAround,
1554                    ..default()
1555                },
1556                DespawnOnExit(super::Scene::DebugOutlines),
1557            ))
1558            .with_children(|parent| {
1559                parent.spawn((
1560                    Node {
1561                        width: px(200),
1562                        height: px(200),
1563                        border: UiRect {
1564                            top: px(10),
1565                            bottom: px(20),
1566                            left: px(30),
1567                            right: px(40),
1568                        },
1569                        border_radius: BorderRadius::bottom_right(px(10)),
1570                        padding: UiRect {
1571                            top: px(40),
1572                            bottom: px(30),
1573                            left: px(20),
1574                            right: px(10),
1575                        },
1576                        ..default()
1577                    },
1578                    children![(
1579                        Text::new("border padding content outlines"),
1580                        TextFont::default(),
1581                        UiDebugOptions {
1582                            enabled: false,
1583                            ..default()
1584                        }
1585                    )],
1586                    UiDebugOptions {
1587                        outline_border_box: true,
1588                        outline_padding_box: true,
1589                        outline_content_box: true,
1590                        ignore_border_radius: false,
1591                        ..debug_options
1592                    },
1593                ));
1594
1595                // Vertical scrollbar (non-functional)
1596                parent.spawn((
1597                    Node {
1598                        flex_direction: FlexDirection::Column,
1599                        width: px(90),
1600                        height: px(230),
1601                        overflow: Overflow::scroll_y(),
1602                        scrollbar_width: 20.,
1603                        ..default()
1604                    },
1605                    ScrollPosition(Vec2::new(180., 180.)),
1606                    UiDebugOptions {
1607                        line_width: 3.,
1608                        outline_scrollbars: true,
1609                        show_hidden: false,
1610                        show_clipped: false,
1611                        ..debug_options
1612                    },
1613                    Children::spawn(SpawnIter((0..20).map(move |i| {
1614                        (
1615                            Node::default(),
1616                            children![(
1617                                Text(format!("Item {i}")),
1618                                UiDebugOptions {
1619                                    enabled: false,
1620                                    ..default()
1621                                }
1622                            )],
1623                            UiDebugOptions {
1624                                enabled: false,
1625                                ..default()
1626                            },
1627                        )
1628                    }))),
1629                ));
1630
1631                // Horizontal scrollbar (non-functional)
1632                parent.spawn((
1633                    Node {
1634                        flex_direction: FlexDirection::Row,
1635                        width: px(156),
1636                        height: px(70),
1637                        overflow: Overflow::scroll_x(),
1638                        scrollbar_width: 10.,
1639                        ..default()
1640                    },
1641                    UiDebugOptions {
1642                        line_width: 3.,
1643                        outline_scrollbars: true,
1644                        show_hidden: false,
1645                        show_clipped: false,
1646                        ..debug_options
1647                    },
1648                    Children::spawn(SpawnIter((0..20).map(move |i| {
1649                        (
1650                            Node::default(),
1651                            children![(
1652                                Text(format!("Item {i}")),
1653                                UiDebugOptions {
1654                                    enabled: false,
1655                                    ..default()
1656                                }
1657                            )],
1658                            UiDebugOptions {
1659                                enabled: false,
1660                                ..default()
1661                            },
1662                        )
1663                    }))),
1664                ));
1665
1666                // bi-directional scrollbar (non-functional)
1667                parent.spawn((
1668                    Node {
1669                        flex_direction: FlexDirection::Column,
1670                        width: px(230),
1671                        height: px(125),
1672                        overflow: Overflow::scroll(),
1673                        scrollbar_width: 20.,
1674                        ..default()
1675                    },
1676                    ScrollPosition(Vec2::new(300., 0.)),
1677                    UiDebugOptions {
1678                        line_width: 3.,
1679                        outline_scrollbars: true,
1680                        show_hidden: false,
1681                        show_clipped: false,
1682                        ..debug_options
1683                    },
1684                    Children::spawn(SpawnIter((0..6).map(move |i| {
1685                        (
1686                            Node {
1687                                flex_direction: FlexDirection::Row,
1688                                ..default()
1689                            },
1690                            Children::spawn(SpawnIter((0..6).map({
1691                                move |j| {
1692                                    (
1693                                        Text(format!("Item {}", (i * 5) + j)),
1694                                        UiDebugOptions {
1695                                            enabled: false,
1696                                            ..default()
1697                                        },
1698                                    )
1699                                }
1700                            }))),
1701                            UiDebugOptions {
1702                                enabled: false,
1703                                ..default()
1704                            },
1705                        )
1706                    }))),
1707                ));
1708            });
1709    }
```

examples/testbed/full\_ui.rs ([line 163](../../src/testbed_full_ui/full_ui.rs.html#163))

```rust
31fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
32    // Camera
33    commands.spawn((Camera2d, IsDefaultUiCamera, BoxShadowSamples(6)));
34
35    // root node
36    commands
37        .spawn(Node {
38            width: percent(100),
39            height: percent(100),
40            justify_content: JustifyContent::SpaceBetween,
41            ..default()
42        })
43        .insert(Pickable::IGNORE)
44        .with_children(|parent| {
45            // left vertical fill (border)
46            parent
47                .spawn((
48                    Node {
49                        width: px(200),
50                        border: UiRect::all(px(2)),
51                        ..default()
52                    },
53                    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
54                ))
55                .with_children(|parent| {
56                    // left vertical fill (content)
57                    parent
58                        .spawn((
59                            Node {
60                                width: percent(100),
61                                flex_direction: FlexDirection::Column,
62                                padding: UiRect::all(px(5)),
63                                row_gap: px(5),
64                                ..default()
65                            },
66                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
67                            Visibility::Visible,
68                        ))
69                        .with_children(|parent| {
70                            // text
71                            parent.spawn((
72                                Text::new("Text Example"),
73                                TextFont {
74                                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
75                                    font_size: FontSize::Px(25.0),
76                                    ..default()
77                                },
78                                // Because this is a distinct label widget and
79                                // not button/list item text, this is necessary
80                                // for accessibility to treat the text accordingly.
81                                Label,
82                            ));
83
84                            #[cfg(feature = "bevy_ui_debug")]
85                            {
86                                // Debug overlay text
87                                parent.spawn((
88                                    Text::new("Press Space to toggle debug outlines."),
89                                    TextFont {
90                                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
91                                        ..default()
92                                    },
93                                    Label,
94                                ));
95
96                                parent.spawn((
97                                    Text::new("V: toggle UI root's visibility"),
98                                    TextFont {
99                                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
100                                        font_size: FontSize::Px(12.),
101                                        ..default()
102                                    },
103                                    Label,
104                                ));
105
106                                parent.spawn((
107                                    Text::new("S: toggle outlines for hidden nodes"),
108                                    TextFont {
109                                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
110                                        font_size: FontSize::Px(12.),
111                                        ..default()
112                                    },
113                                    Label,
114                                ));
115                                parent.spawn((
116                                    Text::new("C: toggle outlines for clipped nodes"),
117                                    TextFont {
118                                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
119                                        font_size: FontSize::Px(12.),
120                                        ..default()
121                                    },
122                                    Label,
123                                ));
124                            }
125                            #[cfg(not(feature = "bevy_ui_debug"))]
126                            parent.spawn((
127                                Text::new("Try enabling feature \"bevy_ui_debug\"."),
128                                TextFont {
129                                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
130                                    ..default()
131                                },
132                                Label,
133                            ));
134                        });
135                });
136            // right vertical fill
137            parent
138                .spawn(Node {
139                    flex_direction: FlexDirection::Column,
140                    justify_content: JustifyContent::Center,
141                    align_items: AlignItems::Center,
142                    width: px(200),
143                    ..default()
144                })
145                .with_children(|parent| {
146                    // Title
147                    parent.spawn((
148                        Text::new("Scrolling list"),
149                        TextFont {
150                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
151                            font_size: FontSize::Px(21.),
152                            ..default()
153                        },
154                        Label,
155                    ));
156                    // Scrolling list
157                    parent
158                        .spawn((
159                            Node {
160                                flex_direction: FlexDirection::Column,
161                                align_self: AlignSelf::Stretch,
162                                height: percent(50),
163                                overflow: Overflow::scroll_y(),
164                                ..default()
165                            },
166                            BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
167                        ))
168                        .with_children(|parent| {
169                            parent
170                                .spawn((
171                                    Node {
172                                        flex_direction: FlexDirection::Column,
173                                        ..Default::default()
174                                    },
175                                    BackgroundGradient::from(LinearGradient::to_bottom(vec![
176                                        ColorStop::auto(NAVY),
177                                        ColorStop::auto(Color::BLACK),
178                                    ])),
179                                    Pickable {
180                                        should_block_lower: false,
181                                        ..Default::default()
182                                    },
183                                ))
184                                .with_children(|parent| {
185                                    // List items
186                                    for i in 0..25 {
187                                        parent
188                                            .spawn((
189                                                Text(format!("Item {i}")),
190                                                TextFont {
191                                                    font: asset_server
192                                                        .load("fonts/FiraSans-Bold.ttf")
193                                                        .into(),
194                                                    ..default()
195                                                },
196                                                Label,
197                                                AccessibilityNode(Accessible::new(Role::ListItem)),
198                                            ))
199                                            .insert(Pickable {
200                                                should_block_lower: false,
201                                                ..default()
202                                            });
203                                    }
204                                });
205                        });
206                });
207
208            parent
209                .spawn(Node {
210                    left: px(210),
211                    bottom: px(10),
212                    position_type: PositionType::Absolute,
213                    ..default()
214                })
215                .with_children(|parent| {
216                    parent
217                        .spawn((
218                            Node {
219                                width: px(200),
220                                height: px(200),
221                                border: UiRect::all(px(20)),
222                                flex_direction: FlexDirection::Column,
223                                justify_content: JustifyContent::Center,
224                                ..default()
225                            },
226                            BorderColor::all(LIME),
227                            BackgroundColor(Color::srgb(0.8, 0.8, 1.)),
228                        ))
229                        .with_children(|parent| {
230                            parent.spawn((
231                                ImageNode::new(asset_server.load("branding/bevy_logo_light.png")),
232                                // Uses the transform to rotate the logo image by 45 degrees
233                                Node {
234                                    border_radius: BorderRadius::all(px(10)),
235                                    ..Default::default()
236                                },
237                                UiTransform {
238                                    rotation: Rot2::radians(0.25 * PI),
239                                    ..Default::default()
240                                },
241                                Outline {
242                                    width: px(2),
243                                    offset: px(4),
244                                    color: DARK_GRAY.into(),
245                                },
246                            ));
247                        });
248                });
249
250            let shadow_style = ShadowStyle {
251                color: Color::BLACK.with_alpha(0.5),
252                blur_radius: px(2),
253                x_offset: px(10),
254                y_offset: px(10),
255                ..default()
256            };
257
258            // render order test: reddest in the back, whitest in the front (flex center)
259            parent
260                .spawn(Node {
261                    width: percent(100),
262                    height: percent(100),
263                    position_type: PositionType::Absolute,
264                    align_items: AlignItems::Center,
265                    justify_content: JustifyContent::Center,
266                    ..default()
267                })
268                .insert(Pickable::IGNORE)
269                .with_children(|parent| {
270                    parent
271                        .spawn((
272                            Node {
273                                width: px(100),
274                                height: px(100),
275                                ..default()
276                            },
277                            BackgroundColor(Color::srgb(1.0, 0.0, 0.)),
278                            BoxShadow::from(shadow_style),
279                        ))
280                        .with_children(|parent| {
281                            parent.spawn((
282                                Node {
283                                    // Take the size of the parent node.
284                                    width: percent(100),
285                                    height: percent(100),
286                                    position_type: PositionType::Absolute,
287                                    left: px(20),
288                                    bottom: px(20),
289                                    ..default()
290                                },
291                                BackgroundColor(Color::srgb(1.0, 0.3, 0.3)),
292                                BoxShadow::from(shadow_style),
293                            ));
294                            parent.spawn((
295                                Node {
296                                    width: percent(100),
297                                    height: percent(100),
298                                    position_type: PositionType::Absolute,
299                                    left: px(40),
300                                    bottom: px(40),
301                                    ..default()
302                                },
303                                BackgroundColor(Color::srgb(1.0, 0.5, 0.5)),
304                                BoxShadow::from(shadow_style),
305                            ));
306                            parent.spawn((
307                                Node {
308                                    width: percent(100),
309                                    height: percent(100),
310                                    position_type: PositionType::Absolute,
311                                    left: px(60),
312                                    bottom: px(60),
313                                    ..default()
314                                },
315                                BackgroundColor(Color::srgb(0.0, 0.7, 0.7)),
316                                BoxShadow::from(shadow_style),
317                            ));
318                            // alpha test
319                            parent.spawn((
320                                Node {
321                                    width: percent(100),
322                                    height: percent(100),
323                                    position_type: PositionType::Absolute,
324                                    left: px(80),
325                                    bottom: px(80),
326                                    ..default()
327                                },
328                                BackgroundColor(Color::srgba(1.0, 0.9, 0.9, 0.4)),
329                                BoxShadow::from(ShadowStyle {
330                                    color: Color::BLACK.with_alpha(0.3),
331                                    ..shadow_style
332                                }),
333                            ));
334                        });
335                });
336            // bevy logo (flex center)
337            parent
338                .spawn(Node {
339                    width: percent(100),
340                    position_type: PositionType::Absolute,
341                    justify_content: JustifyContent::Center,
342                    align_items: AlignItems::FlexStart,
343                    ..default()
344                })
345                .with_children(|parent| {
346                    // bevy logo (image)
347                    parent
348                        .spawn((
349                            ImageNode::new(asset_server.load("branding/bevy_logo_dark_big.png"))
350                                .with_mode(NodeImageMode::Stretch),
351                            Node {
352                                width: px(500),
353                                height: px(125),
354                                margin: UiRect::top(vmin(5)),
355                                ..default()
356                            },
357                        ))
358                        .with_children(|parent| {
359                            // alt text
360                            // This UI node takes up no space in the layout and the `Text` component is used by the accessibility module
361                            // and is not rendered.
362                            parent.spawn((
363                                Node {
364                                    display: Display::None,
365                                    ..default()
366                                },
367                                Text::new("Bevy logo"),
368                            ));
369                        });
370                });
371
372            // four bevy icons demonstrating image flipping
373            parent
374                .spawn(Node {
375                    width: percent(100),
376                    height: percent(100),
377                    position_type: PositionType::Absolute,
378                    justify_content: JustifyContent::Center,
379                    align_items: AlignItems::FlexEnd,
380                    column_gap: px(10),
381                    padding: UiRect::all(px(10)),
382                    ..default()
383                })
384                .insert(Pickable::IGNORE)
385                .with_children(|parent| {
386                    for (flip_x, flip_y) in
387                        [(false, false), (false, true), (true, true), (true, false)]
388                    {
389                        parent.spawn((
390                            ImageNode {
391                                image: asset_server.load("branding/icon.png"),
392                                flip_x,
393                                flip_y,
394                                ..default()
395                            },
396                            Node {
397                                // The height will be chosen automatically to preserve the image's aspect ratio
398                                width: px(75),
399                                ..default()
400                            },
401                        ));
402                    }
403                });
404        });
405}
```

## Trait Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1340)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1341)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1239)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1239)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Overflow](struct.Overflow.html "struct bevy::prelude::Overflow"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Overflow](struct.Overflow.html "struct bevy::prelude::Overflow") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [from\_reflect](trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [get\_represented\_type\_info](trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [try\_apply](trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [reflect\_kind](trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [reflect\_ref](trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [reflect\_mut](trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [reflect\_owned](trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [try\_into\_reflect](trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [try\_as\_reflect](trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [try\_as\_reflect\_mut](trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [into\_partial\_reflect](trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [as\_partial\_reflect](trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [as\_partial\_reflect\_mut](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1236)

#### fn [reflect\_partial\_eq](trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [reflect\_partial\_cmp](trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1236)

#### fn [reflect\_clone](trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#363)

#### fn [debug](trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [into\_any](trait.Reflect.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [as\_any](trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [as\_any\_mut](trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [into\_reflect](trait.Reflect.html#tymethod.into_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [as\_reflect](trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [as\_reflect\_mut](trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [set](trait.Reflect.html#tymethod.set)(&mut self, value: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1239)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1239)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [field](trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [field\_mut](trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [field\_at](trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [field\_at\_mut](trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [name\_at](trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [index\_of\_name](trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [field\_len](trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [iter\_fields](trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [to\_dynamic\_struct](trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [type\_path](trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [short\_type\_path](trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [type\_ident](trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [crate\_name](trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [module\_path](trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

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

[Source](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)

### impl<T> [Brush](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/style/brush/trait.Brush.html "trait parley::style::brush::Brush") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

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

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/mod.rs.html#633)

### impl<T> [DeserializeOwned](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeOwned.html "trait serde_core::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#25-27)

### impl<T> [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#29)

#### fn [dyn\_eq](../app/trait.DynEq.html#tymethod.dyn_eq)(&self, other: &(dyn [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method tests for `self` and `other` values to be equal. [Read more](../app/trait.DynEq.html#tymethod.dyn_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

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

### impl<T> [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#272)

### impl<S> [GetField](trait.GetField.html "trait bevy::prelude::GetField") for S

where S: [Struct](trait.Struct.html "trait bevy::prelude::Struct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#273)

#### fn [get\_field](trait.GetField.html#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#278)

#### fn [get\_field\_mut](trait.GetField.html#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](trait.GetPath.html#method.path_mut)

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

### impl<G> [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

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

### impl<T> [Template](trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](trait.ToOwned.html#method.clone_into)

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

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}