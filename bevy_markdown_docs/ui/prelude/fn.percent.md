[bevy](../../index.html)::[ui](../index.html)::[prelude](index.html)

# Function percent 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#558)

```rust
pub fn percent<T>(value: T) -> Valwhere
    T: ValNum,
```

Returns a [`Val::Percent`](../../prelude/enum.Val.html#variant.Percent "variant bevy::prelude::Val::Percent") representing a percentage of the parent node’s length along a specific axis.

If the UI node has no parent, the percentage is based on the window’s length along that axis.

Axis rules:

*   For `flex_basis`, the percentage is relative to the main-axis length determined by the `flex_direction`.
*   For `gap`, `min_size`, `size`, and `max_size`:
    *   `width` is relative to the parent’s width.
    *   `height` is relative to the parent’s height.
*   For `margin`, `padding`, and `border` values: the percentage is relative to the parent’s width.
*   For positions, `left` and `right` are relative to the parent’s width, while `bottom` and `top` are relative to the parent’s height.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/overflow\_debug.rs ([line 52](../../../src/overflow_debug/overflow_debug.rs.html#52))

```rust
51    fn update(&self, t: f32, transform: &mut UiTransform) {
52        transform.translation.x = percent(ops::sin(t * TAU - FRAC_PI_2) * 50.);
53        transform.translation.y = percent(-ops::cos(t * TAU - FRAC_PI_2) * 50.);
54    }
55}
56
57#[derive(Component)]
58struct Scale;
59
60impl UpdateTransform for Scale {
61    fn update(&self, t: f32, transform: &mut UiTransform) {
62        transform.scale.x = 1.0 + 0.5 * ops::cos(t * TAU).max(0.0);
63        transform.scale.y = 1.0 + 0.5 * ops::cos(t * TAU + PI).max(0.0);
64    }
65}
66
67#[derive(Component)]
68struct Rotate;
69
70impl UpdateTransform for Rotate {
71    fn update(&self, t: f32, transform: &mut UiTransform) {
72        transform.rotation = Rot2::radians(ops::cos(t * TAU) * 45.0);
73    }
74}
75
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
261
262fn next_container_size(mut containers: Query<(&mut Node, &mut Container)>) {
263    for (mut node, mut container) in &mut containers {
264        container.0 = (container.0 + 1) % 3;
265
266        node.width = match container.0 {
267            2 => percent(30),
268            _ => percent(100),
269        };
270        node.height = match container.0 {
271            1 => percent(30),
272            _ => percent(100),
273        };
274    }
275}
```

Hide additional examples

examples/remote/app\_under\_test.rs ([line 66](../../../src/app_under_test/app_under_test.rs.html#66))

```rust
63fn move_button(mut rng: ResMut<SeededRng>, mut button_query: Query<&mut Node, With<Button>>) {
64    let (left_pct, top_pct) = random_position(&mut rng.0);
65    for mut node in &mut button_query {
66        node.left = percent(left_pct);
67        node.top = percent(top_pct);
68    }
69}
70
71fn setup(mut commands: Commands, assets: Res<AssetServer>, mut rng: ResMut<SeededRng>) {
72    let (left_pct, top_pct) = random_position(&mut rng.0);
73
74    commands.spawn(Camera2d);
75    commands
76        .spawn(Node {
77            width: percent(100),
78            height: percent(100),
79            ..default()
80        })
81        .with_children(|parent| {
82            parent
83                .spawn((
84                    Button,
85                    Node {
86                        width: px(150),
87                        height: px(65),
88                        border: UiRect::all(px(5)),
89                        justify_content: JustifyContent::Center,
90                        align_items: AlignItems::Center,
91                        border_radius: BorderRadius::MAX,
92                        left: percent(left_pct),
93                        top: percent(top_pct),
94                        ..default()
95                    },
96                    BorderColor::all(Color::WHITE),
97                    BackgroundColor(Color::BLACK),
98                ))
99                .observe(on_button_click)
100                .with_children(|parent| {
101                    parent.spawn((
102                        Text::new("Button"),
103                        TextFont {
104                            font: assets.load("fonts/FiraSans-Bold.ttf").into(),
105                            font_size: FontSize::Px(33.0),
106                            ..default()
107                        },
108                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
109                        TextShadow::default(),
110                    ));
111                });
112        });
113}
```

examples/window/persisting\_window\_settings.rs ([line 79](../../../src/persisting_window_settings/persisting_window_settings.rs.html#79))

```rust
76fn setup(mut commands: Commands) {
77    commands.spawn((Camera::default(), Camera2d));
78    commands.spawn(Node {
79        width: percent(100),
80        height: percent(100),
81        display: Display::Flex,
82        flex_direction: FlexDirection::Column,
83        align_items: AlignItems::Center,
84        justify_content: JustifyContent::Center,
85        ..default()
86    });
87}
```

examples/window/window\_resizing.rs ([line 39](../../../src/window_resizing/window_resizing.rs.html#39))

```rust
35fn setup_ui(mut commands: Commands) {
36    // Node that fills entire background
37    commands
38        .spawn(Node {
39            width: percent(100),
40            ..default()
41        })
42        // Text where we display current resolution
43        .with_child((
44            Text::new("Resolution"),
45            TextFont {
46                font_size: FontSize::Px(42.0),
47                ..default()
48            },
49            ResolutionText,
50        ));
51}
```

examples/showcase/alien\_cake\_addict.rs ([line 396](../../../src/alien_cake_addict/alien_cake_addict.rs.html#396))

```rust
392fn display_score(mut commands: Commands, game: Res<Game>) {
393    commands.spawn((
394        DespawnOnExit(GameState::GameOver),
395        Node {
396            width: percent(100),
397            align_items: AlignItems::Center,
398            justify_content: JustifyContent::Center,
399            ..default()
400        },
401        children![(
402            Text::new(format!("Cake eaten: {}", game.cake_eaten)),
403            TextFont {
404                font_size: FontSize::Px(67.0),
405                ..default()
406            },
407            TextColor(Color::srgb(0.5, 0.5, 1.0)),
408        )],
409    ));
410}
```

examples/ecs/hotpatching\_systems.rs ([line 63](../../../src/hotpatching_systems/hotpatching_systems.rs.html#63))

```rust
57fn setup(mut commands: Commands) {
58    commands.spawn(Camera2d);
59
60    commands
61        .spawn((
62            Node {
63                width: percent(100),
64                height: percent(100),
65                align_items: AlignItems::Center,
66                justify_content: JustifyContent::Center,
67                flex_direction: FlexDirection::Column,
68                ..default()
69            },
70            children![(
71                Text::default(),
72                TextFont {
73                    font_size: FontSize::Px(100.0),
74                    ..default()
75                },
76            )],
77        ))
78        .observe(on_click);
79}
```

Additional examples can be found in:  

*   [examples/showcase/breakout.rs](../../../src/breakout/breakout.rs.html#59)
*   [examples/showcase/loading\_screen.rs](../../../src/loading_screen/loading_screen.rs.html#251)
*   [examples/window/scale\_factor\_override.rs](../../../src/scale_factor_override/scale_factor_override.rs.html#32)
*   [examples/app/settings.rs](../../../src/settings/settings.rs.html#65)
*   [examples/showcase/game\_menu.rs](../../../src/game_menu/game_menu.rs.html#83)
*   [examples/ui/text/editable\_text\_filter.rs](../../../src/editable_text_filter/editable_text_filter.rs.html#21)
*   [examples/ui/images/image\_node.rs](../../../src/image_node/image_node.rs.html#21)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../../src/standard_widgets_observers/standard_widgets_observers.rs.html#88)
*   [examples/state/custom\_transitions.rs](../../../src/custom_transitions/custom_transitions.rs.html#249)
*   [examples/state/states.rs](../../../src/states/states.rs.html#62)
*   [examples/ui/widgets/button.rs](../../../src/button/button.rs.html#80)
*   [examples/stress\_tests/many\_glyphs.rs](../../../src/many_glyphs/many_glyphs.rs.html#81)
*   [examples/state/sub\_states.rs](../../../src/sub_states/sub_states.rs.html#162)
*   [examples/picking/simple\_picking.rs](../../../src/simple_picking/simple_picking.rs.html#22)
*   [examples/ui/relative\_cursor\_position.rs](../../../src/relative_cursor_position/relative_cursor_position.rs.html#29)
*   [examples/ui/ui\_material.rs](../../../src/ui_material/ui_material.rs.html#30)
*   [examples/picking/debug\_picking.rs](../../../src/debug_picking/debug_picking.rs.html#43)
*   [examples/ui/ui\_scaling.rs](../../../src/ui_scaling/ui_scaling.rs.html#36)
*   [examples/usage/cooldown.rs](../../../src/cooldown/cooldown.rs.html#33)
*   [examples/ui/widgets/standard\_widgets.rs](../../../src/standard_widgets/standard_widgets.rs.html#144)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../../src/ui_texture_atlas/ui_texture_atlas.rs.html#36)
*   [examples/ui/layout/ghost\_nodes.rs](../../../src/ghost_nodes/ghost_nodes.rs.html#41)
*   [examples/stress\_tests/many\_gradients.rs](../../../src/many_gradients/many_gradients.rs.html#87)
*   [examples/stress\_tests/many\_buttons.rs](../../../src/many_buttons/many_buttons.rs.html#180)
*   [examples/usage/context\_menu.rs](../../../src/context_menu/context_menu.rs.html#150)
*   [examples/ui/images/ui\_texture\_slice.rs](../../../src/ui_texture_slice/ui_texture_slice.rs.html#57)
*   [examples/camera/2d\_on\_ui.rs](../../../src/2d_on_ui/2d_on_ui.rs.html#33)
*   [examples/window/monitor\_info.rs](../../../src/monitor_info/monitor_info.rs.html#63)
*   [examples/time/virtual\_time.rs](../../../src/virtual_time/virtual_time.rs.html#83)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#41)
*   [examples/ui/layout/size\_constraints.rs](../../../src/size_constraints/size_constraints.rs.html#57)
*   [examples/ui/text/text\_background\_colors.rs](../../../src/text_background_colors/text_background_colors.rs.html#35)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#157)
*   [examples/picking/dragdrop\_picking.rs](../../../src/dragdrop_picking/dragdrop_picking.rs.html#39)
*   [examples/ui/styling/transparency\_ui.rs](../../../src/transparency_ui/transparency_ui.rs.html#21)
*   [examples/ui/layout/anchor\_layout.rs](../../../src/anchor_layout/anchor_layout.rs.html#101)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#67)
*   [examples/state/computed\_states.rs](../../../src/computed_states/computed_states.rs.html#341)
*   [examples/ui/text/text\_wrap\_debug.rs](../../../src/text_wrap_debug/text_wrap_debug.rs.html#56)
*   [examples/ui/styling/stacked\_gradients.rs](../../../src/stacked_gradients/stacked_gradients.rs.html#21)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#149)
*   [examples/ui/text/system\_fonts.rs](../../../src/system_fonts/system_fonts.rs.html#33)
*   [examples/3d/auto\_exposure.rs](../../../src/auto_exposure/auto_exposure.rs.html#120)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../../src/overflow/overflow.rs.html#23)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../../src/overflow_clip_margin/overflow_clip_margin.rs.html#20)
*   [examples/ui/layout/flex\_layout.rs](../../../src/flex_layout/flex_layout.rs.html#28)
*   [examples/ui/widgets/vertical\_slider.rs](../../../src/vertical_slider/vertical_slider.rs.html#42)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#300)
*   [examples/ui/layout/display\_and\_visibility.rs](../../../src/display_and_visibility/display_and_visibility.rs.html#86)
*   [examples/ui/widgets/tab\_navigation.rs](../../../src/tab_navigation/tab_navigation.rs.html#73)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../../src/drag_to_scroll/drag_to_scroll.rs.html#32)
*   [examples/3d/split\_screen.rs](../../../src/split_screen/split_screen.rs.html#88)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../../src/scroll/scroll.rs.html#120)
*   [examples/ui/layout/z\_index.rs](../../../src/z_index/z_index.rs.html#27)
*   [examples/ui/render\_ui\_to\_texture.rs](../../../src/render_ui_to_texture/render_ui_to_texture.rs.html#80)
*   [examples/ui/images/image\_node\_resizing.rs](../../../src/image_node_resizing/image_node_resizing.rs.html#75)
*   [examples/ui/text/strikethrough\_and\_underline.rs](../../../src/strikethrough_and_underline/strikethrough_and_underline.rs.html#39)
*   [examples/ui/navigation/directional\_navigation.rs](../../../src/directional_navigation/directional_navigation.rs.html#120)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../../src/multiple_text_inputs/multiple_text_inputs.rs.html#55)
*   [examples/ui/styling/box\_shadow.rs](../../../src/box_shadow/box_shadow.rs.html#150)
*   [examples/usage/debug\_frustum\_culling.rs](../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#145)
*   [examples/ui/text/letter\_spacing.rs](../../../src/letter_spacing/letter_spacing.rs.html#37)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#170)
*   [examples/ui/text/text\_debug.rs](../../../src/text_debug/text_debug.rs.html#40)
*   [examples/ui/layout/grid.rs](../../../src/grid/grid.rs.html#29)
*   [examples/ui/ui\_transform.rs](../../../src/ui_transform/ui_transform.rs.html#109)
*   [examples/ui/text/multiline\_text\_input.rs](../../../src/multiline_text_input/multiline_text_input.rs.html#33)
*   [examples/ui/styling/gradients.rs](../../../src/gradients/gradients.rs.html#42)
*   [examples/testbed/full\_ui.rs](../../../src/testbed_full_ui/full_ui.rs.html#38)