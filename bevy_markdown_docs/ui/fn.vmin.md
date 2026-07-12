[bevy](../index.html)::[ui](index.html)

# Function vmin 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#573)

```rust
pub fn vmin<T>(value: T) -> Valwhere
    T: ValNum,
```

Returns a [`Val::VMin`](../prelude/enum.Val.html#variant.VMin "variant bevy::prelude::Val::VMin") representing a percentage of the viewport’s smaller dimension.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/stress\_tests/many\_buttons.rs ([line 166](../../src/many_buttons/many_buttons.rs.html#166))

```rust
152fn setup_flex(mut commands: Commands, asset_server: Res<AssetServer>, args: Res<Args>) {
153    let images = if 0 < args.image_freq {
154        Some(vec![
155            asset_server.load("branding/icon.png"),
156            asset_server.load("textures/Game Icons/wrench.png"),
157        ])
158    } else {
159        None
160    };
161
162    let buttons_f = args.buttons as f32;
163    let border = if args.no_borders {
164        UiRect::ZERO
165    } else {
166        UiRect::all(vmin(0.05 * 90. / buttons_f))
167    };
168
169    let as_rainbow = |i: usize| Color::hsl((i as f32 / buttons_f) * 360.0, 0.9, 0.8);
170    commands
171        .spawn(Node {
172            display: if args.display_none {
173                Display::None
174            } else {
175                Display::Flex
176            },
177            flex_direction: FlexDirection::Column,
178            justify_content: JustifyContent::Center,
179            align_items: AlignItems::Center,
180            width: percent(100),
181            height: percent(100),
182            ..default()
183        })
184        .with_children(|commands| {
185            for column in 0..args.buttons {
186                commands.spawn(Node::default()).with_children(|commands| {
187                    for row in 0..args.buttons {
188                        let color = as_rainbow(row % column.max(1));
189                        let border_color = Color::WHITE.with_alpha(0.5).into();
190                        spawn_button(
191                            commands,
192                            color,
193                            buttons_f,
194                            column,
195                            row,
196                            args.text,
197                            border,
198                            border_color,
199                            images.as_ref().map(|images| {
200                                images[((column + row) / args.image_freq) % images.len()].clone()
201                            }),
202                        );
203                    }
204                });
205            }
206        });
207}
208
209fn setup_grid(mut commands: Commands, asset_server: Res<AssetServer>, args: Res<Args>) {
210    let images = if 0 < args.image_freq {
211        Some(vec![
212            asset_server.load("branding/icon.png"),
213            asset_server.load("textures/Game Icons/wrench.png"),
214        ])
215    } else {
216        None
217    };
218
219    let buttons_f = args.buttons as f32;
220    let border = if args.no_borders {
221        UiRect::ZERO
222    } else {
223        UiRect::all(vmin(0.05 * 90. / buttons_f))
224    };
225
226    let as_rainbow = |i: usize| Color::hsl((i as f32 / buttons_f) * 360.0, 0.9, 0.8);
227    commands
228        .spawn(Node {
229            display: if args.display_none {
230                Display::None
231            } else {
232                Display::Grid
233            },
234            width: percent(100),
235            height: percent(100),
236            grid_template_columns: RepeatedGridTrack::flex(args.buttons as u16, 1.0),
237            grid_template_rows: RepeatedGridTrack::flex(args.buttons as u16, 1.0),
238            ..default()
239        })
240        .with_children(|commands| {
241            for column in 0..args.buttons {
242                for row in 0..args.buttons {
243                    let color = as_rainbow(row % column.max(1));
244                    let border_color = Color::WHITE.with_alpha(0.5).into();
245                    spawn_button(
246                        commands,
247                        color,
248                        buttons_f,
249                        column,
250                        row,
251                        args.text,
252                        border,
253                        border_color,
254                        images.as_ref().map(|images| {
255                            images[((column + row) / args.image_freq) % images.len()].clone()
256                        }),
257                    );
258                }
259            }
260        });
261}
262
263fn spawn_button(
264    commands: &mut ChildSpawnerCommands,
265    background_color: Color,
266    buttons: f32,
267    column: usize,
268    row: usize,
269    spawn_text: bool,
270    border: UiRect,
271    border_color: BorderColor,
272    image: Option<Handle<Image>>,
273) {
274    let width = vw(90.0 / buttons);
275    let height = vh(90.0 / buttons);
276    let margin = UiRect::axes(width * 0.05, height * 0.05);
277    let mut builder = commands.spawn((
278        Button,
279        Node {
280            width,
281            height,
282            margin,
283            align_items: AlignItems::Center,
284            justify_content: JustifyContent::Center,
285            border,
286            ..default()
287        },
288        BackgroundColor(background_color),
289        border_color,
290        IdleColor(background_color),
291    ));
292
293    if let Some(image) = image {
294        builder.insert(ImageNode::new(image));
295    }
296
297    if spawn_text {
298        builder.with_children(|parent| {
299            // These labels are split to stress test multi-span text
300            parent
301                .spawn((
302                    Text(format!("{column}, ")),
303                    TextFont {
304                        font_size: FONT_SIZE,
305                        ..default()
306                    },
307                    TextColor(Color::srgb(0.5, 0.2, 0.2)),
308                ))
309                .with_child((
310                    TextSpan(format!("{row}")),
311                    TextFont {
312                        font_size: FONT_SIZE,
313                        ..default()
314                    },
315                    TextColor(Color::srgb(0.2, 0.2, 0.5)),
316                ));
317        });
318    }
319}
320
321fn despawn_ui(mut commands: Commands, root_node: Single<Entity, (With<Node>, Without<ChildOf>)>) {
322    commands.entity(*root_node).despawn();
323}
324
325fn setup_many_cameras(mut commands: Commands, asset_server: Res<AssetServer>, args: Res<Args>) {
326    let images = if 0 < args.image_freq {
327        Some(vec![
328            asset_server.load("branding/icon.png"),
329            asset_server.load("textures/Game Icons/wrench.png"),
330        ])
331    } else {
332        None
333    };
334
335    let buttons_f = args.buttons as f32;
336    let border = if args.no_borders {
337        UiRect::ZERO
338    } else {
339        UiRect::all(vmin(0.05 * 90. / buttons_f))
340    };
341
342    let as_rainbow = |i: usize| Color::hsl((i as f32 / buttons_f) * 360.0, 0.9, 0.8);
343    for column in 0..args.buttons {
344        for row in 0..args.buttons {
345            let color = as_rainbow(row % column.max(1));
346            let border_color = Color::WHITE.with_alpha(0.5).into();
347            let camera = commands
348                .spawn((
349                    Camera2d,
350                    Camera {
351                        order: (column * args.buttons + row) as isize + 1,
352                        ..Default::default()
353                    },
354                ))
355                .id();
356            commands
357                .spawn((
358                    Node {
359                        display: if args.display_none {
360                            Display::None
361                        } else {
362                            Display::Flex
363                        },
364                        flex_direction: FlexDirection::Column,
365                        justify_content: JustifyContent::Center,
366                        align_items: AlignItems::Center,
367                        width: percent(100),
368                        height: percent(100),
369                        ..default()
370                    },
371                    UiTargetCamera(camera),
372                ))
373                .with_children(|commands| {
374                    commands
375                        .spawn(Node {
376                            position_type: PositionType::Absolute,
377                            top: vh(column as f32 * 100. / buttons_f),
378                            left: vw(row as f32 * 100. / buttons_f),
379                            ..Default::default()
380                        })
381                        .with_children(|commands| {
382                            spawn_button(
383                                commands,
384                                color,
385                                buttons_f,
386                                column,
387                                row,
388                                args.text,
389                                border,
390                                border_color,
391                                images.as_ref().map(|images| {
392                                    images[((column + row) / args.image_freq) % images.len()]
393                                        .clone()
394                                }),
395                            );
396                        });
397                });
398        }
399    }
400}
```

Hide additional examples

examples/testbed/ui.rs ([line 1740](../../src/testbed_ui/ui.rs.html#1740))

```rust
1721    pub fn setup(mut commands: Commands) {
1722        commands.spawn((Camera2d, DespawnOnExit(super::Scene::ViewportCoords)));
1723        commands
1724            .spawn((
1725                Node {
1726                    width: vw(100),
1727                    height: vh(100),
1728                    border: UiRect::axes(vw(5), vh(5)),
1729                    flex_wrap: FlexWrap::Wrap,
1730                    ..default()
1731                },
1732                BorderColor::all(PALETTE[0]),
1733                DespawnOnExit(super::Scene::ViewportCoords),
1734            ))
1735            .with_children(|builder| {
1736                builder.spawn((
1737                    Node {
1738                        width: vw(30),
1739                        height: vh(30),
1740                        border: UiRect::all(vmin(5)),
1741                        ..default()
1742                    },
1743                    BackgroundColor(PALETTE[1].into()),
1744                    BorderColor::all(PALETTE[8]),
1745                ));
1746
1747                builder.spawn((
1748                    Node {
1749                        width: vw(60),
1750                        height: vh(30),
1751                        ..default()
1752                    },
1753                    BackgroundColor(PALETTE[2].into()),
1754                ));
1755
1756                builder.spawn((
1757                    Node {
1758                        width: vw(45),
1759                        height: vh(30),
1760                        border: UiRect::left(vmax(45. / 2.)),
1761                        ..default()
1762                    },
1763                    BackgroundColor(PALETTE[3].into()),
1764                    BorderColor::all(PALETTE[7]),
1765                ));
1766
1767                builder.spawn((
1768                    Node {
1769                        width: vw(45),
1770                        height: vh(30),
1771                        border: UiRect::right(vmax(45. / 2.)),
1772                        ..default()
1773                    },
1774                    BackgroundColor(PALETTE[4].into()),
1775                    BorderColor::all(PALETTE[7]),
1776                ));
1777
1778                builder.spawn((
1779                    Node {
1780                        width: vw(60),
1781                        height: vh(30),
1782                        ..default()
1783                    },
1784                    BackgroundColor(PALETTE[5].into()),
1785                ));
1786
1787                builder.spawn((
1788                    Node {
1789                        width: vw(30),
1790                        height: vh(30),
1791                        border: UiRect::all(vmin(5)),
1792                        ..default()
1793                    },
1794                    BackgroundColor(PALETTE[6].into()),
1795                    BorderColor::all(PALETTE[8]),
1796                ));
1797            });
1798    }
```

examples/testbed/full\_ui.rs ([line 354](../../src/testbed_full_ui/full_ui.rs.html#354))

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