[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Struct EntityCommands 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1304)

```rust
pub struct EntityCommands<'a> { /* private fields */ }
```

A list of commands that will be run to modify an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

## Note

Most [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") (and thereby [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")) are deferred: when you call the command, if it requires mutable access to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") (that is, if it removes, adds, or changes something), it’s not executed immediately.

Instead, the command is added to a “command queue.” The command queue is applied later when the [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred") system runs. Commands are executed one-by-one so that each command can have exclusive access to the `World`.

## Fallible

Due to their deferred nature, an entity you’re trying to change with an [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") can be despawned by the time the command is executed.

All deferred entity commands will check whether the entity exists at the time of execution and will return an error if it doesn’t.

## Error handling

An [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") can return a [`Result`](../../prelude/type.Result.html "type bevy::prelude::Result"), which will be passed to an [error handler](../error/index.html "mod bevy::ecs::error") if the `Result` is an error.

The fallback error handler panics. It can be configured via the [`FallbackErrorHandler`](../error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler") resource.

Alternatively, you can customize the error handler for a specific command by calling [`EntityCommands::queue_handled`](../../prelude/struct.EntityCommands.html#method.queue_handled "method bevy::prelude::EntityCommands::queue_handled").

The [`error`](../error/index.html "mod bevy::ecs::error") module provides some simple error handlers for convenience.

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#381)

### impl<'a> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#383-386)

#### pub fn [with\_children](#method.with_children)( &mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [RelatedSpawnerCommands](../relationship/struct.RelatedSpawnerCommands.html "struct bevy::ecs::relationship::RelatedSpawnerCommands")<'\_, [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Spawns children of this entity (with a [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship) by taking a function that operates on a [`ChildSpawner`](../../prelude/type.ChildSpawner.html "type bevy::prelude::ChildSpawner").

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ecs/observer\_propagation.rs ([lines 31-41](../../../src/observer_propagation/observer_propagation.rs.html#31-41))

```rust
27fn setup(mut commands: Commands) {
28    commands
29        .spawn((Name::new("Goblin"), HitPoints(50)))
30        .observe(take_damage)
31        .with_children(|parent| {
32            parent
33                .spawn((Name::new("Helmet"), Armor(5)))
34                .observe(block_attack);
35            parent
36                .spawn((Name::new("Socks"), Armor(10)))
37                .observe(block_attack);
38            parent
39                .spawn((Name::new("Shirt"), Armor(15)))
40                .observe(block_attack);
41        });
42}
```

Hide additional examples

examples/app/log\_layers\_ecs.rs ([lines 151-164](../../../src/log_layers_ecs/log_layers_ecs.rs.html#151-164))

```rust
144fn print_logs(
145    mut log_message_reader: MessageReader<LogMessage>,
146    mut commands: Commands,
147    log_viewer_root: Single<Entity, With<LogViewerRoot>>,
148) {
149    let root_entity = *log_viewer_root;
150
151    commands.entity(root_entity).with_children(|child| {
152        for log_message in log_message_reader.read() {
153            child.spawn((
154                Text::default(),
155                children![
156                    (
157                        TextSpan::new(format!("{:5} ", log_message.level)),
158                        TextColor(level_color(&log_message.level)),
159                    ),
160                    TextSpan::new(&log_message.message),
161                ],
162            ));
163        }
164    });
165}
```

examples/app/settings.rs ([lines 73-90](../../../src/settings/settings.rs.html#73-90))

```rust
61fn setup(mut commands: Commands) {
62    commands.spawn((Camera::default(), Camera2d));
63    commands
64        .spawn(Node {
65            width: percent(100),
66            height: percent(100),
67            display: Display::Flex,
68            flex_direction: FlexDirection::Column,
69            align_items: AlignItems::Center,
70            justify_content: JustifyContent::Center,
71            ..default()
72        })
73        .with_children(|parent| {
74            parent.spawn((
75                Text::new("---"),
76                TextFont {
77                    font_size: FontSize::Px(33.0),
78                    ..default()
79                },
80                CounterDisplay,
81                TextColor(Color::srgb(0.9, 0.9, 0.9)),
82            ));
83            parent.spawn((
84                Text::new("Press SPACE to increment, BACKSPACE to decrement."),
85                TextFont {
86                    font_size: FontSize::Px(20.0),
87                    ..default()
88                },
89            ));
90        });
91}
```

examples/ui/text/editable\_text\_filter.rs ([lines 27-46](../../../src/editable_text_filter/editable_text_filter.rs.html#27-46))

```rust
16fn setup(mut commands: Commands) {
17    commands.spawn(Camera2d);
18
19    commands
20        .spawn(Node {
21            width: percent(100.),
22            height: percent(100.),
23            justify_content: JustifyContent::Center,
24            align_items: AlignItems::Center,
25            ..default()
26        })
27        .with_children(|parent| {
28            parent.spawn((
29                Node {
30                    width: px(240.),
31                    border: px(2.).all(),
32                    padding: px(8.).all(),
33                    ..default()
34                },
35                EditableText {
36                    max_characters: Some(8),
37                    ..default()
38                },
39                TextCursorStyle::default(),
40                EditableTextFilter::new(|c| c.is_ascii_hexdigit()),
41                TextFont::from_font_size(32.),
42                BackgroundColor(DARK_SLATE_GRAY.into()),
43                BorderColor::all(SLATE_300),
44                AutoFocus,
45            ));
46        });
47}
```

examples/ui/text/font\_atlas\_debug.rs ([lines 97-107](../../../src/font_atlas_debug/font_atlas_debug.rs.html#97-107))

```rust
84fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<State>) {
85    state.handle = asset_server.load("fonts/FiraSans-Bold.ttf");
86    let font = FontSource::from(state.handle.clone());
87    commands.spawn(Camera2d);
88    commands
89        .spawn((
90            Node {
91                position_type: PositionType::Absolute,
92                bottom: Val::ZERO,
93                ..default()
94            },
95            BackgroundColor(Color::NONE),
96        ))
97        .with_children(|parent| {
98            parent.spawn((
99                Text::new("a"),
100                TextFont {
101                    font,
102                    font_size: FontSize::Px(50.0),
103                    ..default()
104                },
105                TextColor(YELLOW.into()),
106            ));
107        });
108    // We're seeding the PRNG here to make this example deterministic for testing purposes.
109    // This isn't strictly required in practical use unless you need your app to be deterministic.
110    commands.insert_resource(SeededRng(ChaCha8Rng::seed_from_u64(19878367467713)));
111}
```

examples/stress\_tests/many\_glyphs.rs ([lines 86-93](../../../src/many_glyphs/many_glyphs.rs.html#86-93))

```rust
64fn setup(mut commands: Commands, args: Res<Args>) {
65    warn!(include_str!("warning_string.txt"));
66
67    commands.spawn(Camera2d);
68    let text_string = "0123456789".repeat(10_000);
69    let text_font = TextFont {
70        font_size: FontSize::Px(4.),
71        ..Default::default()
72    };
73    let text_block = TextLayout {
74        justify: Justify::Left,
75        linebreak: LineBreak::AnyCharacter,
76    };
77
78    if !args.no_ui {
79        commands
80            .spawn(Node {
81                width: percent(100),
82                align_items: AlignItems::Center,
83                justify_content: JustifyContent::Center,
84                ..default()
85            })
86            .with_children(|commands| {
87                commands
88                    .spawn(Node {
89                        width: px(1000),
90                        ..Default::default()
91                    })
92                    .with_child((Text(text_string.clone()), text_font.clone(), text_block));
93            });
94    }
95
96    if !args.no_text2d {
97        commands.spawn((
98            Text2d::new(text_string),
99            text_font.clone(),
100            TextColor(RED.into()),
101            bevy::sprite::Anchor::CENTER,
102            TextBounds::new_horizontal(1000.),
103            text_block,
104        ));
105    }
106}
```

Additional examples can be found in:  

*   [examples/stress\_tests/text\_pipeline.rs](../../../src/text_pipeline/text_pipeline.rs.html#73-77)
*   [examples/ecs/hierarchy.rs](../../../src/hierarchy/hierarchy.rs.html#30-40)
*   [examples/ui/relative\_cursor\_position.rs](../../../src/relative_cursor_position/relative_cursor_position.rs.html#36-58)
*   [examples/ui/ui\_material.rs](../../../src/ui_material/ui_material.rs.html#36-60)
*   [examples/remote/app\_under\_test.rs](../../../src/app_under_test/app_under_test.rs.html#81-112)
*   [examples/ui/ui\_scaling.rs](../../../src/ui_scaling/ui_scaling.rs.html#47-76)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../../src/ui_texture_atlas/ui_texture_atlas.rs.html#44-66)
*   [examples/ui/layout/ghost\_nodes.rs](../../../src/ghost_nodes/ghost_nodes.rs.html#31-36)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../../src/overflow_debug/overflow_debug.rs.html#114-133)
*   [examples/stress\_tests/many\_gradients.rs](../../../src/many_gradients/many_gradients.rs.html#94-129)
*   [examples/stress\_tests/many\_buttons.rs](../../../src/many_buttons/many_buttons.rs.html#184-206)
*   [examples/ui/images/ui\_texture\_slice.rs](../../../src/ui_texture_slice/ui_texture_slice.rs.html#63-94)
*   [examples/math/cubic\_splines.rs](../../../src/cubic_splines/cubic_splines.rs.html#89-93)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#50-70)
*   [examples/ui/layout/size\_constraints.rs](../../../src/size_constraints/size_constraints.rs.html#65-107)
*   [examples/ui/text/text\_background\_colors.rs](../../../src/text_background_colors/text_background_colors.rs.html#41-84)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#164-202)
*   [examples/ui/text/generic\_font\_families.rs](../../../src/generic_font_families/generic_font_families.rs.html#46-98)
*   [examples/picking/dragdrop\_picking.rs](../../../src/dragdrop_picking/dragdrop_picking.rs.html#47-80)
*   [examples/ui/styling/transparency\_ui.rs](../../../src/transparency_ui/transparency_ui.rs.html#27-79)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#73-113)
*   [examples/picking/sprite\_picking.rs](../../../src/sprite_picking/sprite_picking.rs.html#39-89)
*   [examples/3d/ssr.rs](../../../src/ssr/ssr.rs.html#445-510)
*   [examples/ui/styling/stacked\_gradients.rs](../../../src/stacked_gradients/stacked_gradients.rs.html#26-90)
*   [examples/gizmos/transform\_gizmo.rs](../../../src/transform_gizmo/transform_gizmo.rs.html#67-93)
*   [examples/ui/text/system\_fonts.rs](../../../src/system_fonts/system_fonts.rs.html#41-95)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../../src/overflow/overflow.rs.html#31-94)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../../src/overflow_clip_margin/overflow_clip_margin.rs.html#30-91)
*   [examples/ui/layout/flex\_layout.rs](../../../src/flex_layout/flex_layout.rs.html#38-108)
*   [examples/ui/widgets/vertical\_slider.rs](../../../src/vertical_slider/vertical_slider.rs.html#53-131)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#269-296)
*   [examples/ui/layout/display\_and\_visibility.rs](../../../src/display_and_visibility/display_and_visibility.rs.html#95-163)
*   [examples/ui/widgets/tab\_navigation.rs](../../../src/tab_navigation/tab_navigation.rs.html#88-149)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../../src/drag_to_scroll/drag_to_scroll.rs.html#64-114)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../../src/scroll/scroll.rs.html#126-204)
*   [examples/ui/layout/z\_index.rs](../../../src/z_index/z_index.rs.html#33-119)
*   [examples/ui/render\_ui\_to\_texture.rs](../../../src/render_ui_to_texture/render_ui_to_texture.rs.html#90-131)
*   [examples/ui/images/image\_node\_resizing.rs](../../../src/image_node_resizing/image_node_resizing.rs.html#119-175)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#136-201)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#179-188)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../../src/multiple_text_inputs/multiple_text_inputs.rs.html#68-172)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#25-99)
*   [examples/ui/text/text.rs](../../../src/text/text.rs.html#103-145)
*   [examples/ui/text/letter\_spacing.rs](../../../src/letter_spacing/letter_spacing.rs.html#41-200)
*   [examples/ui/text/text\_debug.rs](../../../src/text_debug/text_debug.rs.html#55-97)
*   [examples/ui/layout/grid.rs](../../../src/grid/grid.rs.html#48-176)
*   [examples/ui/text/multiline\_text\_input.rs](../../../src/multiline_text_input/multiline_text_input.rs.html#39-273)
*   [examples/ui/styling/gradients.rs](../../../src/gradients/gradients.rs.html#37-285)
*   [examples/testbed/full\_ui.rs](../../../src/testbed_full_ui/full_ui.rs.html#44-404)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#392)

#### pub fn [add\_children](#method.add_children)(&mut self, children: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\]) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Adds the given children to this entity.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/ui/text/text\_input.rs ([line 111](../../../src/text_input/text_input.rs.html#111))

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

Hide additional examples

examples/ui/text/ime\_support.rs ([line 94](../../../src/ime_support/ime_support.rs.html#94))

```rust
30fn setup(mut commands: Commands) {
31    commands.spawn(Camera2d);
32
33    let instructions = commands
34        .spawn((
35            Text::new("Type using your IME, then press Ctrl+Enter to submit. Your system default sans-serif font will be used, so make sure you have fonts installed that support the characters you want to input!"),
36            TextFont {
37                font_size: FontSize::Px(20.0),
38                ..default()
39            },
40        ))
41        .id();
42
43    let text_input = commands
44        .spawn((
45            Node {
46                width: px(400),
47                height: px(250),
48                border: px(3).all(),
49                padding: px(8).all(),
50                ..default()
51            },
52            // SansSerif resolves to a system sans-serif font, which on most CJK systems
53            // includes support for Chinese, Japanese, and Korean characters.
54            // Note that using system fonts requires the "bevy/system-fonts" feature to be enabled.
55            TextFont {
56                font: FontSource::SansSerif,
57                font_size: FontSize::Px(32.0),
58                ..default()
59            },
60            BorderColor::from(Color::from(SLATE_300)),
61            EditableText {
62                allow_newlines: true,
63                ..default()
64            },
65            TextLayout::no_wrap(),
66            TextCursorStyle::default(),
67            TabIndex(0),
68            BackgroundColor(DARK_GREY.into()),
69        ))
70        .id();
71
72    let text_output = commands
73        .spawn((
74            Text::new("Your text here!"),
75            TextFont {
76                font: FontSource::SansSerif,
77                font_size: FontSize::Px(32.0),
78                ..default()
79            },
80            TextOutput,
81        ))
82        .id();
83
84    commands
85        .spawn((
86            Node {
87                flex_direction: FlexDirection::Column,
88                padding: px(24.0).all(),
89                row_gap: px(16),
90                ..default()
91            },
92            TabGroup::new(0),
93        ))
94        .add_children(&[instructions, text_input, text_output]);
95}
```

examples/ui/navigation/directional\_navigation.rs ([line 255](../../../src/directional_navigation/directional_navigation.rs.html#255))

```rust
114fn setup_scattered_ui(mut commands: Commands, mut input_focus: ResMut<InputFocus>) {
115    commands.spawn(Camera2d);
116
117    // Create a full-screen background node
118    let root_node = commands
119        .spawn(Node {
120            width: percent(100),
121            height: percent(100),
122            ..default()
123        })
124        .id();
125
126    // Instructions
127    let instructions = commands
128        .spawn((
129            Text::new(
130                "Directional Navigation Demo\n\n\
131                 Use arrow keys or D-pad to navigate.\n\
132                 Press Enter or A button to interact.\n\n\
133                 Buttons are scattered irregularly,\n\
134                 but navigation is automatic!",
135            ),
136            Node {
137                position_type: PositionType::Absolute,
138                left: px(20),
139                top: px(20),
140                width: px(280),
141                padding: UiRect::all(px(12)),
142                border_radius: BorderRadius::all(px(8)),
143                ..default()
144            },
145            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
146        ))
147        .id();
148
149    // Focus display - shows which button is currently focused
150    commands.spawn((
151        Text::new("Focused: None"),
152        FocusDisplay,
153        Node {
154            position_type: PositionType::Absolute,
155            left: px(20),
156            bottom: px(80),
157            width: px(280),
158            padding: UiRect::all(px(12)),
159            border_radius: BorderRadius::all(px(8)),
160            ..default()
161        },
162        BackgroundColor(Color::srgba(0.1, 0.5, 0.1, 0.8)),
163        TextFont {
164            font_size: FontSize::Px(20.0),
165            ..default()
166        },
167    ));
168
169    // Key display - shows the last key pressed
170    commands.spawn((
171        Text::new("Last Key: None"),
172        KeyDisplay,
173        Node {
174            position_type: PositionType::Absolute,
175            left: px(20),
176            bottom: px(20),
177            width: px(280),
178            padding: UiRect::all(px(12)),
179            border_radius: BorderRadius::all(px(8)),
180            ..default()
181        },
182        BackgroundColor(Color::srgba(0.5, 0.1, 0.5, 0.8)),
183        TextFont {
184            font_size: FontSize::Px(20.0),
185            ..default()
186        },
187    ));
188
189    // Spawn buttons in a scattered/irregular pattern
190    // The auto-navigation system will figure out the connections!
191    let button_positions = [
192        // Top row (irregular spacing)
193        (350.0, 100.0),
194        (520.0, 120.0),
195        (700.0, 90.0),
196        // Middle-top row
197        (380.0, 220.0),
198        (600.0, 240.0),
199        // Center
200        (450.0, 340.0),
201        (620.0, 360.0),
202        // Lower row
203        (360.0, 480.0),
204        (540.0, 460.0),
205        (720.0, 490.0),
206    ];
207
208    let mut first_button = None;
209    for (i, (x, y)) in button_positions.iter().enumerate() {
210        let transform = if i == 4 {
211            UiTransform {
212                scale: Vec2::splat(1.2),
213                rotation: Rot2::FRAC_PI_2,
214                ..default()
215            }
216        } else {
217            UiTransform::IDENTITY
218        };
219        let button_entity = commands
220            .spawn((
221                Button,
222                Node {
223                    position_type: PositionType::Absolute,
224                    left: px(*x),
225                    top: px(*y),
226                    width: px(140),
227                    height: px(80),
228                    border: UiRect::all(px(4)),
229                    justify_content: JustifyContent::Center,
230                    align_items: AlignItems::Center,
231                    border_radius: BorderRadius::all(px(12)),
232                    ..default()
233                },
234                transform,
235                // This is the key: just add this component for automatic navigation!
236                AutoDirectionalNavigation::default(),
237                ResetTimer::default(),
238                BackgroundColor::from(NORMAL_BUTTON),
239                Name::new(format!("Button {}", i + 1)),
240            ))
241            .with_child((
242                Text::new(format!("Button {}", i + 1)),
243                TextLayout {
244                    justify: Justify::Center,
245                    ..default()
246                },
247            ))
248            .id();
249
250        if first_button.is_none() {
251            first_button = Some(button_entity);
252        }
253    }
254
255    commands.entity(root_node).add_children(&[instructions]);
256
257    // Set initial focus
258    if let Some(button) = first_button {
259        input_focus.set(button, FocusCause::Navigated);
260    }
261}
```

examples/animation/custom\_skinned\_mesh.rs ([line 165](../../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#165))

```rust
38fn setup(
39    mut commands: Commands,
40    asset_server: Res<AssetServer>,
41    mut meshes: ResMut<Assets<Mesh>>,
42    mut materials: ResMut<Assets<StandardMaterial>>,
43    mut skinned_mesh_inverse_bindposes_assets: ResMut<Assets<SkinnedMeshInverseBindposes>>,
44) {
45    // Create a camera
46    commands.spawn((
47        Camera3d::default(),
48        Transform::from_xyz(2.5, 2.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
49    ));
50
51    // Create inverse bindpose matrices for a skeleton consists of 2 joints
52    let inverse_bindposes = skinned_mesh_inverse_bindposes_assets.add(vec![
53        Mat4::from_translation(Vec3::new(-0.5, -1.0, 0.0)),
54        Mat4::from_translation(Vec3::new(-0.5, -1.0, 0.0)),
55    ]);
56
57    // Create a mesh
58    let mesh = Mesh::new(
59        PrimitiveTopology::TriangleList,
60        RenderAssetUsages::RENDER_WORLD,
61    )
62    // Set mesh vertex positions
63    .with_inserted_attribute(
64        Mesh::ATTRIBUTE_POSITION,
65        vec![
66            [0.0, 0.0, 0.0],
67            [1.0, 0.0, 0.0],
68            [0.0, 0.5, 0.0],
69            [1.0, 0.5, 0.0],
70            [0.0, 1.0, 0.0],
71            [1.0, 1.0, 0.0],
72            [0.0, 1.5, 0.0],
73            [1.0, 1.5, 0.0],
74            [0.0, 2.0, 0.0],
75            [1.0, 2.0, 0.0],
76        ],
77    )
78    // Add UV coordinates that map the left half of the texture since its a 1 x
79    // 2 rectangle.
80    .with_inserted_attribute(
81        Mesh::ATTRIBUTE_UV_0,
82        vec![
83            [0.0, 0.00],
84            [0.5, 0.00],
85            [0.0, 0.25],
86            [0.5, 0.25],
87            [0.0, 0.50],
88            [0.5, 0.50],
89            [0.0, 0.75],
90            [0.5, 0.75],
91            [0.0, 1.00],
92            [0.5, 1.00],
93        ],
94    )
95    // Set mesh vertex normals
96    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 10])
97    // Set mesh vertex joint indices for mesh skinning.
98    // Each vertex gets 4 indices used to address the `JointTransforms` array in the vertex shader
99    //  as well as `SkinnedMeshJoint` array in the `SkinnedMesh` component.
100    // This means that a maximum of 4 joints can affect a single vertex.
101    .with_inserted_attribute(
102        Mesh::ATTRIBUTE_JOINT_INDEX,
103        // Need to be explicit here as [u16; 4] could be either Uint16x4 or Unorm16x4.
104        VertexAttributeValues::Uint16x4(vec![
105            [0, 0, 0, 0],
106            [0, 0, 0, 0],
107            [0, 1, 0, 0],
108            [0, 1, 0, 0],
109            [0, 1, 0, 0],
110            [0, 1, 0, 0],
111            [0, 1, 0, 0],
112            [0, 1, 0, 0],
113            [0, 1, 0, 0],
114            [0, 1, 0, 0],
115        ]),
116    )
117    // Set mesh vertex joint weights for mesh skinning.
118    // Each vertex gets 4 joint weights corresponding to the 4 joint indices assigned to it.
119    // The sum of these weights should equal to 1.
120    .with_inserted_attribute(
121        Mesh::ATTRIBUTE_JOINT_WEIGHT,
122        vec![
123            [1.00, 0.00, 0.0, 0.0],
124            [1.00, 0.00, 0.0, 0.0],
125            [0.75, 0.25, 0.0, 0.0],
126            [0.75, 0.25, 0.0, 0.0],
127            [0.50, 0.50, 0.0, 0.0],
128            [0.50, 0.50, 0.0, 0.0],
129            [0.25, 0.75, 0.0, 0.0],
130            [0.25, 0.75, 0.0, 0.0],
131            [0.00, 1.00, 0.0, 0.0],
132            [0.00, 1.00, 0.0, 0.0],
133        ],
134    )
135    // Tell bevy to construct triangles from a list of vertex indices,
136    // where each 3 vertex indices form a triangle.
137    .with_inserted_indices(Indices::U16(vec![
138        0, 1, 3, 0, 3, 2, 2, 3, 5, 2, 5, 4, 4, 5, 7, 4, 7, 6, 6, 7, 9, 6, 9, 8,
139    ]))
140    // Create skinned mesh bounds. Together with the `DynamicSkinnedMeshBounds`
141    // component, this will ensure the mesh is correctly frustum culled.
142    .with_generated_skinned_mesh_bounds()
143    .unwrap();
144
145    let mesh = meshes.add(mesh);
146
147    // We're seeding the PRNG here to make this example deterministic for testing purposes.
148    // This isn't strictly required in practical use unless you need your app to be deterministic.
149    let mut rng = ChaCha8Rng::seed_from_u64(42);
150
151    for i in -5..5 {
152        // Create joint entities
153        let joint_0 = commands
154            .spawn(Transform::from_xyz(
155                i as f32 * 1.5,
156                0.0,
157                // Move quads back a small amount to avoid Z-fighting and not
158                // obscure the transform gizmos.
159                -(i as f32 * 0.01).abs(),
160            ))
161            .id();
162        let joint_1 = commands.spawn((AnimatedJoint(i), Transform::IDENTITY)).id();
163
164        // Set joint_1 as a child of joint_0.
165        commands.entity(joint_0).add_children(&[joint_1]);
166
167        // Each joint in this vector corresponds to each inverse bindpose matrix in `SkinnedMeshInverseBindposes`.
168        let joint_entities = vec![joint_0, joint_1];
169
170        // Create skinned mesh renderer. Note that its transform doesn't affect the position of the mesh.
171        commands.spawn((
172            Mesh3d(mesh.clone()),
173            MeshMaterial3d(materials.add(StandardMaterial {
174                base_color: Color::srgb(
175                    rng.random_range(0.0..1.0),
176                    rng.random_range(0.0..1.0),
177                    rng.random_range(0.0..1.0),
178                ),
179                base_color_texture: Some(asset_server.load("textures/uv_checker_bw.png")),
180                ..default()
181            })),
182            SkinnedMesh {
183                inverse_bindposes: inverse_bindposes.clone(),
184                joints: joint_entities,
185            },
186            DynamicSkinnedMeshBounds,
187        ));
188    }
189}
```

examples/ui/navigation/directional\_navigation\_overrides.rs ([line 198](../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#198))

```rust
160fn setup_paged_ui(
161    mut commands: Commands,
162    mut manual_directional_nav_map: ResMut<DirectionalNavigationMap>,
163    mut input_focus: ResMut<InputFocus>,
164) {
165    commands.spawn(Camera2d);
166
167    // Create a full-screen background node
168    let root_node = commands
169        .spawn(Node {
170            width: percent(100),
171            height: percent(100),
172            ..default()
173        })
174        .id();
175
176    // Instructions
177    let instructions = commands
178        .spawn((
179            Text::new(
180                "Directional Navigation Overrides Demo\n\n\
181                 Use arrow keys or D-pad to navigate.\n\
182                 Press Enter or A button to interact.\n\n\
183                 Navigation on each page is a combination of \
184                 both automatic and manual navigation.",
185            ),
186            Node {
187                position_type: PositionType::Absolute,
188                left: px(20),
189                top: px(20),
190                width: px(280),
191                padding: UiRect::all(px(12)),
192                border_radius: BorderRadius::all(px(8)),
193                ..default()
194            },
195            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
196        ))
197        .id();
198    commands.entity(root_node).add_children(&[instructions]);
199
200    // Focus display - shows which button is currently focused
201    commands.spawn((
202        Text::new("Focused: None"),
203        FocusDisplay,
204        Node {
205            position_type: PositionType::Absolute,
206            left: px(20),
207            bottom: px(80),
208            width: px(280),
209            padding: UiRect::all(px(12)),
210            border_radius: BorderRadius::all(px(8)),
211            ..default()
212        },
213        BackgroundColor(Color::srgba(0.1, 0.5, 0.1, 0.8)),
214        TextFont {
215            font_size: FontSize::Px(20.0),
216            ..default()
217        },
218    ));
219
220    // Key display - shows the last key pressed
221    commands.spawn((
222        Text::new("Last Key: None"),
223        KeyDisplay,
224        Node {
225            position_type: PositionType::Absolute,
226            left: px(20),
227            bottom: px(20),
228            width: px(280),
229            padding: UiRect::all(px(12)),
230            border_radius: BorderRadius::all(px(8)),
231            ..default()
232        },
233        BackgroundColor(Color::srgba(0.5, 0.1, 0.5, 0.8)),
234        TextFont {
235            font_size: FontSize::Px(20.0),
236            ..default()
237        },
238    ));
239
240    // Setup the pages with buttons and helper text
241    let mut pages_entities = [
242        Vec::with_capacity(12),
243        Vec::with_capacity(12),
244        Vec::with_capacity(12),
245    ];
246    let mut text_entities = Vec::with_capacity(10);
247    for (page_num, page_button_entities) in pages_entities.iter_mut().enumerate() {
248        if page_num == 1 {
249            // the second page
250            setup_buttons_for_triangle_page(
251                &mut commands,
252                page_num,
253                (page_button_entities, &mut text_entities),
254            );
255        } else {
256            // the first and third pages are regular grids
257            setup_buttons_for_grid_page(
258                &mut commands,
259                page_num,
260                (page_button_entities, &mut text_entities),
261            );
262        }
263
264        // Only the first page is visible at setup.
265        let visibility = if page_num == 0 {
266            Visibility::Visible
267        } else {
268            Visibility::Hidden
269        };
270        let page = commands
271            .spawn((
272                Node {
273                    width: percent(100),
274                    height: percent(100),
275                    ..default()
276                },
277                visibility,
278            ))
279            .id();
280
281        commands
282            .entity(page)
283            .add_children(page_button_entities)
284            .add_children(&text_entities);
285
286        text_entities.clear();
287    }
288
289    // For Pages 1 and 3, add manual edges within the grid page for navigation between rows.
290    let entity_pairs = [
291        // the end of the first row should connect to the beginning of the second
292        ((0, 2), (1, 0)),
293        // the end of the second row should connect to the beginning of the third
294        ((1, 2), (2, 0)),
295        // the end of the third row should connect to the beginning of the fourth
296        ((2, 2), (3, 0)),
297    ];
298    for (page_num, page_entities) in pages_entities.iter().enumerate() {
299        // Skip Page 2; we are only adding these manual edges for the grid pages.
300        if page_num == 1 {
301            continue;
302        }
303        for ((entity_a_row, entity_a_col), (entity_b_row, entity_b_col)) in entity_pairs.iter() {
304            manual_directional_nav_map.add_symmetrical_edge(
305                page_entities[entity_a_row * 3 + entity_a_col],
306                page_entities[entity_b_row * 3 + entity_b_col],
307                CompassOctant::East,
308            );
309        }
310    }
311
312    // Add manual edges within the triangle page (Page 2) between buttons 3 and 4.
313    // The `AutoNavigationConfig` is set to our desired values, but automatic
314    // navigation does not connect Button 3 to Button 4, so we have to add
315    // this navigation manually.
316    manual_directional_nav_map.add_symmetrical_edge(
317        pages_entities[1][2],
318        pages_entities[1][3],
319        CompassOctant::East,
320    );
321    manual_directional_nav_map.add_symmetrical_edge(
322        pages_entities[1][2],
323        pages_entities[1][3],
324        CompassOctant::South,
325    );
326    manual_directional_nav_map.add_symmetrical_edge(
327        pages_entities[1][2],
328        pages_entities[1][3],
329        CompassOctant::SouthEast,
330    );
331    // Add one-way blocking within the first grid page (Page 1) for down nav.
332    for btn in &pages_entities[0] {
333        manual_directional_nav_map.block_edge(*btn, CompassOctant::South);
334        manual_directional_nav_map.block_edge(*btn, CompassOctant::North);
335    }
336
337    // For Page 3, we override the navigation North and South to be inverted.
338    let mut col_entities = Vec::with_capacity(4);
339    for col in 0..=2 {
340        for row in 0..=3 {
341            col_entities.push(pages_entities[2][row * 3 + col]);
342        }
343        manual_directional_nav_map.add_looping_edges(&col_entities, CompassOctant::North);
344        col_entities.clear();
345    }
346
347    // Add manual edges between pages.
348    // When navigating east (right) from the last button of page 1,
349    // go to the first button of page 2. This edge is symmetrical.
350    manual_directional_nav_map.add_symmetrical_edge(
351        pages_entities[0][11],
352        pages_entities[1][0],
353        CompassOctant::East,
354    );
355    // When navigating south (down) from the last button of page 2,
356    // go to the first button of page 3. This edge is NOT symmetrical.
357    // This means going north (up) from the first button of page 3 does
358    // NOT go to the last button of page 2.
359    manual_directional_nav_map.add_edge(
360        pages_entities[1][3],
361        pages_entities[2][0],
362        CompassOctant::South,
363    );
364    // When navigating west (left) from the first button of page 3,
365    // go back to the last button of page 2. This edge is NOT symmetrical.
366    manual_directional_nav_map.add_edge(
367        pages_entities[2][0],
368        pages_entities[1][3],
369        CompassOctant::West,
370    );
371    // When navigating east (right) from the last button of page 1,
372    // go to the first button of page 2. This edge is symmetrical.
373    manual_directional_nav_map.add_symmetrical_edge(
374        pages_entities[2][11],
375        pages_entities[0][0],
376        CompassOctant::East,
377    );
378
379    // Set initial focus
380    input_focus.set(pages_entities[0][0], FocusCause::Navigated);
381}
```

examples/ui/text/text\_debug.rs ([line 245](../../../src/text_debug/text_debug.rs.html#245))

```rust
33fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
34    let font = FontSource::from(asset_server.load("fonts/FiraSans-Bold.ttf"));
35    let background_color = MAROON.into();
36    commands.spawn(Camera2d);
37
38    let root_uinode = commands
39        .spawn(Node {
40            width: percent(100),
41            height: percent(100),
42            justify_content: JustifyContent::SpaceBetween,
43            ..default()
44        })
45        .id();
46
47    let left_column = commands
48        .spawn(Node {
49            flex_direction: FlexDirection::Column,
50            justify_content: JustifyContent::SpaceBetween,
51            align_items: AlignItems::Start,
52            flex_grow: 1.,
53            margin: UiRect::axes(px(15), px(5)),
54            ..default()
55        }).with_children(|builder| {
56        builder.spawn((
57            Text::new("This is\ntext with\nline breaks\nin the top left."),
58            TextFont {
59                font: font.clone(),
60                font_size: FontSize::Px(25.0),
61                ..default()
62            },
63            BackgroundColor(background_color)
64        ));
65        builder.spawn((
66            Text::new(
67                "This text is right-justified. The `Justify` component controls the horizontal alignment of the lines of multi-line text relative to each other, and does not affect the text node's position in the UI layout.",
68            ),
69            TextFont {
70                font: font.clone(),
71                font_size: FontSize::Px(25.0),
72                ..default()
73            },
74            TextColor(YELLOW.into()),
75            TextLayout::justify(Justify::Right),
76            Node {
77                max_width: px(300),
78                ..default()
79            },
80            BackgroundColor(background_color)
81        ));
82        builder.spawn((
83            Text::new(
84                "This\ntext has\nline breaks and also a set width in the bottom left."),
85            TextFont {
86                font: font.clone(),
87                font_size: FontSize::Px(25.0),
88                ..default()
89            },
90            Node {
91                max_width: px(300),
92                ..default()
93            },
94            BackgroundColor(background_color)
95        )
96        );
97    }).id();
98
99    let right_column = commands
100        .spawn(Node {
101            flex_direction: FlexDirection::Column,
102            justify_content: JustifyContent::SpaceBetween,
103            align_items: AlignItems::End,
104            flex_grow: 1.,
105            margin: UiRect::axes(px(15), px(5)),
106            ..default()
107        })
108        .with_children(|builder| {
109            builder.spawn((
110                Text::new("This text is very long, has a limited width, is center-justified, is positioned in the top right and is also colored pink."),
111                TextFont {
112                    font: font.clone(),
113                    font_size: FontSize::Px(33.0),
114                    ..default()
115                },
116                TextColor(Color::srgb(0.8, 0.2, 0.7)),
117                TextLayout::justify(Justify::Center),
118                Node {
119                    max_width: px(400),
120                    ..default()
121                },
122                BackgroundColor(background_color),
123            ));
124
125            builder.spawn((
126                Text::new("This text is left-justified and is vertically positioned to distribute the empty space equally above and below it."),
127                TextFont {
128                    font: font.clone(),
129                    font_size: FontSize::Px(29.0),
130                    ..default()
131                },
132                TextColor(YELLOW.into()),
133                TextLayout::justify(Justify::Left),
134                Node {
135                    max_width: px(300),
136                    ..default()
137                },
138                BackgroundColor(background_color),
139            ));
140
141            builder.spawn((
142                Text::new("This text is fully justified and is positioned in the same way."),
143                TextFont {
144                    font: font.clone(),
145                    font_size: FontSize::Px(29.0),
146                    ..default()
147                },
148                TextLayout::justify(Justify::Justified),
149                TextColor(GREEN_YELLOW.into()),
150                Node {
151                    max_width: px(300),
152                    ..default()
153                },
154                BackgroundColor(background_color),
155            ));
156
157            builder
158                .spawn((
159                    Text::default(),
160                    TextFont {
161                        font: font.clone(),
162                        font_size: FontSize::Px(21.0),
163                        ..default()
164                    },
165                    TextChanges,
166                    BackgroundColor(background_color),
167                ))
168                .with_children(|p| {
169                    p.spawn((
170                        TextSpan::new("\nThis text changes in the bottom right"),
171                        TextFont {
172                            font: font.clone(),
173                            font_size: FontSize::Px(21.0),
174                            ..default()
175                        },
176                    ));
177                    p.spawn((
178                        TextSpan::new(" this text has zero font size"),
179                        TextFont {
180                            font: font.clone(),
181                            font_size: FontSize::Px(0.0),
182                            ..default()
183                        },
184                        TextColor(BLUE.into()),
185                    ));
186                    p.spawn((
187                        TextSpan::new("\nThis text changes in the bottom right - "),
188                        TextFont {
189                            font: font.clone(),
190                            font_size: FontSize::Px(21.0),
191                            ..default()
192                        },
193                        TextColor(RED.into()),
194                    ));
195                    p.spawn((
196                        TextSpan::default(),
197                        TextFont {
198                            font: font.clone(),
199                            font_size: FontSize::Px(21.0),
200                            ..default()
201                        },
202                        TextColor(ORANGE_RED.into()),
203                    ));
204                    p.spawn((
205                        TextSpan::new(" fps, "),
206                        TextFont {
207                            font: font.clone(),
208                            font_size: FontSize::Px(10.0),
209                            ..default()
210                        },
211                        TextColor(YELLOW.into()),
212                    ));
213                    p.spawn((
214                        TextSpan::default(),
215                        TextFont {
216                            font: font.clone(),
217                            font_size: FontSize::Px(21.0),
218                            ..default()
219                        },
220                        TextColor(LIME.into()),
221                    ));
222                    p.spawn((
223                        TextSpan::new(" ms/frame"),
224                        TextFont {
225                            font: font.clone(),
226                            font_size: FontSize::Px(42.0),
227                            ..default()
228                        },
229                        TextColor(BLUE.into()),
230                    ));
231                    p.spawn((
232                        TextSpan::new(" this text has negative font size"),
233                        TextFont {
234                            font: font.clone(),
235                            font_size: FontSize::Px(-42.0),
236                            ..default()
237                        },
238                        TextColor(BLUE.into()),
239                    ));
240                });
241        })
242        .id();
243    commands
244        .entity(root_uinode)
245        .add_children(&[left_column, right_column]);
246}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#399)

#### pub fn [clear\_children](#method.clear_children)(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

👎Deprecated:

Use detach\_all\_children() instead

Removes all the children from this entity. See also [`detach_all_related`](../../prelude/struct.EntityCommands.html#method.detach_all_related "method bevy::prelude::EntityCommands::detach_all_related")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#406)

#### pub fn [detach\_all\_children](#method.detach_all_children)(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Removes all the parent-child relationships from this entity. To despawn the child entities, instead use [`EntityWorldMut::despawn_children`](../../prelude/struct.EntityWorldMut.html#method.despawn_children "method bevy::prelude::EntityWorldMut::despawn_children"). See also [`detach_all_related`](../../prelude/struct.EntityCommands.html#method.detach_all_related "method bevy::prelude::EntityCommands::detach_all_related")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#412)

#### pub fn [insert\_children](#method.insert_children)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), children: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Insert children at specific index. See also [`insert_related`](../../prelude/struct.EntityCommands.html#method.insert_related "method bevy::prelude::EntityCommands::insert_related").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#418)

#### pub fn [insert\_child](#method.insert_child)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), child: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Insert children at specific index. See also [`insert_related`](../../prelude/struct.EntityCommands.html#method.insert_related "method bevy::prelude::EntityCommands::insert_related").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#423)

#### pub fn [add\_child](#method.add_child)(&mut self, child: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Adds the given child to this entity.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/3d/ssr.rs ([line 319](../../../src/ssr/ssr.rs.html#319))

```rust
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
```

Hide additional examples

examples/ecs/hierarchy.rs ([line 58](../../../src/hierarchy/hierarchy.rs.html#58))

```rust
19fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
20    commands.spawn(Camera2d);
21    let texture = asset_server.load("branding/icon.png");
22
23    // Spawn a root entity with no parent
24    let parent = commands
25        .spawn((
26            Sprite::from_image(texture.clone()),
27            Transform::from_scale(Vec3::splat(0.75)),
28        ))
29        // With that entity as a parent, run a lambda that spawns its children
30        .with_children(|parent| {
31            // parent is a ChildSpawnerCommands, which has a similar API to Commands
32            parent.spawn((
33                Transform::from_xyz(250.0, 0.0, 0.0).with_scale(Vec3::splat(0.75)),
34                Sprite {
35                    image: texture.clone(),
36                    color: BLUE.into(),
37                    ..default()
38                },
39            ));
40        })
41        // Store parent entity for next sections
42        .id();
43
44    // Another way is to use the add_child function to add children after the parent
45    // entity has already been spawned.
46    let child = commands
47        .spawn((
48            Sprite {
49                image: texture,
50                color: LIME.into(),
51                ..default()
52            },
53            Transform::from_xyz(0.0, 250.0, 0.0).with_scale(Vec3::splat(0.75)),
54        ))
55        .id();
56
57    // Add child to the parent.
58    commands.entity(parent).add_child(child);
59}
```

examples/ui/widgets/standard\_widgets.rs ([line 866](../../../src/standard_widgets/standard_widgets.rs.html#866))

```rust
819fn spawn_menu(anchor: Entity, assets: Res<AssetServer>, mut commands: Commands) {
820    let menu = commands
821        .spawn((
822            Node {
823                display: Display::Flex,
824                flex_direction: FlexDirection::Column,
825                min_height: px(10.),
826                min_width: percent(100),
827                border: UiRect::all(px(1)),
828                position_type: PositionType::Absolute,
829                ..default()
830            },
831            MenuPopup::default(),
832            BorderColor::all(GREEN),
833            BackgroundColor(GRAY.into()),
834            BoxShadow::new(
835                Srgba::BLACK.with_alpha(0.9).into(),
836                px(0),
837                px(0),
838                px(1),
839                px(4),
840            ),
841            GlobalZIndex(100),
842            Popover {
843                positions: vec![
844                    PopoverPlacement {
845                        side: PopoverSide::Bottom,
846                        align: PopoverAlign::Start,
847                        gap: 2.0,
848                    },
849                    PopoverPlacement {
850                        side: PopoverSide::Top,
851                        align: PopoverAlign::Start,
852                        gap: 2.0,
853                    },
854                ],
855                window_margin: 10.0,
856            },
857            OverrideClip,
858            children![
859                menu_item(&assets),
860                menu_item(&assets),
861                menu_item(&assets),
862                menu_item(&assets)
863            ],
864        ))
865        .id();
866    commands.entity(anchor).add_child(menu);
867}
```

examples/3d/occlusion\_culling.rs ([line 301](../../../src/occlusion_culling/occlusion_culling.rs.html#301))

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

examples/animation/animation\_graph.rs ([line 335](../../../src/animation_graph/animation_graph.rs.html#335))

```rust
271fn setup_node_rects(commands: &mut Commands) {
272    for (node_rect, node_type) in NODE_RECTS.iter().zip(NODE_TYPES.iter()) {
273        let node_string = match *node_type {
274            NodeType::Clip(ref clip) => clip.text,
275            NodeType::Blend(text) => text,
276        };
277
278        let text = commands
279            .spawn((
280                Text::new(node_string),
281                TextFont {
282                    font_size: FontSize::Px(16.0),
283                    ..default()
284                },
285                TextColor(ANTIQUE_WHITE.into()),
286                TextLayout::justify(Justify::Center),
287            ))
288            .id();
289
290        let container = {
291            let mut container = commands.spawn((
292                Node {
293                    position_type: PositionType::Absolute,
294                    bottom: px(node_rect.bottom),
295                    left: px(node_rect.left),
296                    height: px(node_rect.height),
297                    width: px(node_rect.width),
298                    align_items: AlignItems::Center,
299                    justify_items: JustifyItems::Center,
300                    align_content: AlignContent::Center,
301                    justify_content: JustifyContent::Center,
302                    ..default()
303                },
304                BorderColor::all(WHITE),
305                Outline::new(px(1), Val::ZERO, Color::WHITE),
306            ));
307
308            if let NodeType::Clip(clip) = node_type {
309                container.insert((
310                    Interaction::None,
311                    RelativeCursorPosition::default(),
312                    (*clip).clone(),
313                ));
314            }
315
316            container.id()
317        };
318
319        // Create the background color.
320        if let NodeType::Clip(_) = node_type {
321            let background = commands
322                .spawn((
323                    Node {
324                        position_type: PositionType::Absolute,
325                        top: px(0),
326                        left: px(0),
327                        height: px(node_rect.height),
328                        width: px(node_rect.width),
329                        ..default()
330                    },
331                    BackgroundColor(DARK_GREEN.into()),
332                ))
333                .id();
334
335            commands.entity(container).add_child(background);
336        }
337
338        commands.entity(container).add_child(text);
339    }
340}
```

examples/3d/irradiance\_volumes.rs ([line 592](../../../src/irradiance_volumes/irradiance_volumes.rs.html#592))

```rust
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

Additional examples can be found in:  

*   [examples/stress\_tests/transform\_hierarchy.rs](../../../src/transform_hierarchy/transform_hierarchy.rs.html#430)
*   [examples/ui/text/text\_wrap\_debug.rs](../../../src/text_wrap_debug/text_wrap_debug.rs.html#123)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#828)
*   [examples/ui/ui\_target\_camera.rs](../../../src/ui_target_camera/ui_target_camera.rs.html#92)
*   [examples/3d/contact\_shadows.rs](../../../src/contact_shadows/contact_shadows.rs.html#179)
*   [examples/ui/styling/gradients.rs](../../../src/gradients/gradients.rs.html#284)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#429)

#### pub fn [remove\_children](#method.remove_children)( &mut self, children: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

👎Deprecated:

Use detach\_children() instead

Removes the relationship between this entity and the given entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#435)

#### pub fn [detach\_children](#method.detach_children)( &mut self, children: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Removes the parent-child relationship between this entity and the given entities. Does not despawn the children.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#441)

#### pub fn [remove\_child](#method.remove_child)(&mut self, child: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

👎Deprecated:

Use detach\_child() instead

Removes the relationship between this entity and the given entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#447)

#### pub fn [detach\_child](#method.detach_child)(&mut self, child: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Removes the parent-child relationship between this entity and the given entity. Does not despawn the child.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#452)

#### pub fn [replace\_children](#method.replace_children)( &mut self, children: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Replaces the children on this entity with a new list of children.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#466-471)

#### pub fn [replace\_children\_with\_difference](#method.replace_children_with_difference)( &mut self, entities\_to\_unrelate: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], entities\_to\_relate: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], newly\_related\_entities: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Replaces all the related entities with a new set of entities.

##### Warning

Failing to maintain the functions invariants may lead to erratic engine behavior including random crashes. Refer to [`EntityWorldMut::replace_related_with_difference`](../../prelude/struct.EntityWorldMut.html#method.replace_related_with_difference "method bevy::prelude::EntityWorldMut::replace_related_with_difference") for a list of these invariants.

##### Panics

Panics when debug assertions are enabled if an invariant is broken and the command is executed.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#484)

#### pub fn [with\_child](#method.with_child)(&mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Spawns the passed bundle and adds it to this entity as a child.

For efficient spawning of multiple children, use [`with_children`](../../prelude/struct.EntityCommands.html#method.with_children "method bevy::prelude::EntityCommands::with_children").

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/window/window\_resizing.rs ([lines 43-50](../../../src/window_resizing/window_resizing.rs.html#43-50))

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

Hide additional examples

examples/camera/first\_person\_view\_model.rs ([lines 198-202](../../../src/first_person_view_model/first_person_view_model.rs.html#198-202))

```rust
190fn spawn_text(mut commands: Commands) {
191    commands
192        .spawn(Node {
193            position_type: PositionType::Absolute,
194            bottom: px(12),
195            left: px(12),
196            ..default()
197        })
198        .with_child(Text::new(concat!(
199            "Move the camera with your mouse.\n",
200            "Press arrow up to decrease the FOV of the world model.\n",
201            "Press arrow down to increase the FOV of the world model."
202        )));
203}
```

examples/showcase/contributors.rs ([lines 160-166](../../../src/contributors/contributors.rs.html#160-166))

```rust
139fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
140    commands.spawn(Camera2d);
141
142    let text_style = TextFont {
143        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
144        font_size: FontSize::Px(60.0),
145        ..default()
146    };
147
148    commands
149        .spawn((
150            Text::new("Contributor showcase"),
151            text_style.clone(),
152            ContributorDisplay,
153            Node {
154                position_type: PositionType::Absolute,
155                top: px(12),
156                left: px(12),
157                ..default()
158            },
159        ))
160        .with_child((
161            TextSpan::default(),
162            TextFont {
163                font_size: FontSize::Px(30.),
164                ..text_style
165            },
166        ));
167}
```

examples/showcase/loading\_screen.rs ([line 94](../../../src/loading_screen/loading_screen.rs.html#94))

```rust
72fn setup(mut commands: Commands) {
73    let level_data = LevelData {
74        unload_level_id: commands.register_system(unload_current_level),
75        level_1_id: commands.register_system(load_level_1),
76        level_2_id: commands.register_system(load_level_2),
77    };
78    commands.insert_resource(level_data);
79
80    // Spawns the UI that will show the user prompts.
81    let text_style = TextFont {
82        font_size: FontSize::Px(42.0),
83        ..default()
84    };
85    commands
86        .spawn((
87            Node {
88                justify_self: JustifySelf::Center,
89                align_self: AlignSelf::FlexEnd,
90                ..default()
91            },
92            BackgroundColor(Color::NONE),
93        ))
94        .with_child((Text::new("Press 1 or 2 to load a new scene."), text_style));
95}
96
97// Selects the level you want to load.
98fn level_selection(
99    mut commands: Commands,
100    keyboard: Res<ButtonInput<KeyCode>>,
101    level_data: Res<LevelData>,
102    loading_state: Res<LoadingState>,
103) {
104    // Only trigger a load if the current level is fully loaded.
105    if let LoadingState::LevelReady = loading_state.as_ref() {
106        if keyboard.just_pressed(KeyCode::Digit1) {
107            commands.run_system(level_data.unload_level_id);
108            commands.run_system(level_data.level_1_id);
109        } else if keyboard.just_pressed(KeyCode::Digit2) {
110            commands.run_system(level_data.unload_level_id);
111            commands.run_system(level_data.level_2_id);
112        }
113    }
114}
115
116// Marker component for easier deletion of entities.
117#[derive(Component)]
118struct LevelComponents;
119
120// Removes all currently loaded level assets from the game World.
121fn unload_current_level(
122    mut commands: Commands,
123    mut loading_state: ResMut<LoadingState>,
124    entities: Query<Entity, With<LevelComponents>>,
125) {
126    *loading_state = LoadingState::LevelLoading;
127    for entity in entities.iter() {
128        commands.entity(entity).despawn();
129    }
130}
131
132fn load_level_1(
133    mut commands: Commands,
134    mut loading_data: ResMut<LoadingData>,
135    asset_server: Res<AssetServer>,
136) {
137    // Spawn the camera.
138    commands.spawn((
139        Camera3d::default(),
140        Transform::from_xyz(155.0, 155.0, 155.0).looking_at(Vec3::new(0.0, 40.0, 0.0), Vec3::Y),
141        LevelComponents,
142    ));
143
144    // Save the asset into the `loading_assets` vector.
145    let fox = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/animated/Fox.glb"));
146    loading_data.loading_assets.push(fox.clone().into());
147    // Spawn the fox.
148    commands.spawn((
149        WorldAssetRoot(fox.clone()),
150        Transform::from_xyz(0.0, 0.0, 0.0),
151        LevelComponents,
152    ));
153
154    // Spawn the light.
155    commands.spawn((
156        DirectionalLight {
157            shadow_maps_enabled: true,
158            ..default()
159        },
160        Transform::from_xyz(3.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
161        LevelComponents,
162    ));
163}
164
165fn load_level_2(
166    mut commands: Commands,
167    mut loading_data: ResMut<LoadingData>,
168    asset_server: Res<AssetServer>,
169) {
170    // Spawn the camera.
171    commands.spawn((
172        Camera3d::default(),
173        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::new(0.0, 0.2, 0.0), Vec3::Y),
174        LevelComponents,
175    ));
176
177    // Spawn the helmet.
178    let helmet_scene = asset_server
179        .load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf"));
180    loading_data
181        .loading_assets
182        .push(helmet_scene.clone().into());
183    commands.spawn((WorldAssetRoot(helmet_scene.clone()), LevelComponents));
184
185    // Spawn the light.
186    commands.spawn((
187        DirectionalLight {
188            shadow_maps_enabled: true,
189            ..default()
190        },
191        Transform::from_xyz(3.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
192        LevelComponents,
193    ));
194}
195
196// Monitors current loading status of assets.
197fn update_loading_data(
198    mut loading_data: ResMut<LoadingData>,
199    mut loading_state: ResMut<LoadingState>,
200    asset_server: Res<AssetServer>,
201    pipelines_ready: Res<PipelinesReady>,
202) {
203    if !loading_data.loading_assets.is_empty() || !pipelines_ready.0 {
204        // If we are still loading assets / pipelines are not fully compiled,
205        // we reset the confirmation frame count.
206        loading_data.confirmation_frames_count = 0;
207
208        loading_data.loading_assets.retain(|asset| {
209            asset_server
210                .get_recursive_dependency_load_state(asset)
211                .is_none_or(|state| !state.is_loaded())
212        });
213
214        // If there are no more assets being monitored, and pipelines
215        // are compiled, then start counting confirmation frames.
216        // Once enough confirmations have passed, everything will be
217        // considered to be fully loaded.
218    } else {
219        loading_data.confirmation_frames_count += 1;
220        if loading_data.confirmation_frames_count == loading_data.confirmation_frames_target {
221            *loading_state = LoadingState::LevelReady;
222        }
223    }
224}
225
226// Marker tag for loading screen components.
227#[derive(Component)]
228struct LoadingScreen;
229
230// Spawns the necessary components for the loading screen.
231fn load_loading_screen(mut commands: Commands) {
232    let text_style = TextFont {
233        font_size: FontSize::Px(67.0),
234        ..default()
235    };
236
237    // Spawn the UI and Loading screen camera.
238    commands.spawn((
239        Camera2d,
240        Camera {
241            order: 1,
242            ..default()
243        },
244        LoadingScreen,
245    ));
246
247    // Spawn the UI that will make up the loading screen.
248    commands
249        .spawn((
250            Node {
251                height: percent(100),
252                width: percent(100),
253                justify_content: JustifyContent::Center,
254                align_items: AlignItems::Center,
255                ..default()
256            },
257            BackgroundColor(Color::BLACK),
258            LoadingScreen,
259        ))
260        .with_child((Text::new("Loading..."), text_style.clone()));
261}
```

examples/ui/navigation/directional\_navigation\_overrides.rs ([lines 595-601](../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#595-601))

```rust
566fn spawn_auto_nav_button(
567    commands: &mut Commands,
568    text: String,
569    left: &f64,
570    top: &f64,
571    page_num: usize,
572) -> Entity {
573    commands
574        .spawn((
575            Button,
576            Node {
577                position_type: PositionType::Absolute,
578                left: px(*left),
579                top: px(*top),
580                width: px(140),
581                height: px(100),
582                border: UiRect::all(px(4)),
583                justify_content: JustifyContent::Center,
584                align_items: AlignItems::Center,
585                border_radius: BorderRadius::all(px(12)),
586                ..default()
587            },
588            Page(page_num),
589            BackgroundColor(NORMAL_BUTTON_COLORS[page_num].into()),
590            // Just add this component for automatic navigation
591            AutoDirectionalNavigation::default(),
592            ResetTimer::default(),
593            Name::new(text.clone()),
594        ))
595        .with_child((
596            Text::new(text),
597            TextLayout {
598                justify: Justify::Center,
599                ..default()
600            },
601        ))
602        .id()
603}
```

examples/stress\_tests/many\_glyphs.rs ([line 92](../../../src/many_glyphs/many_glyphs.rs.html#92))

```rust
64fn setup(mut commands: Commands, args: Res<Args>) {
65    warn!(include_str!("warning_string.txt"));
66
67    commands.spawn(Camera2d);
68    let text_string = "0123456789".repeat(10_000);
69    let text_font = TextFont {
70        font_size: FontSize::Px(4.),
71        ..Default::default()
72    };
73    let text_block = TextLayout {
74        justify: Justify::Left,
75        linebreak: LineBreak::AnyCharacter,
76    };
77
78    if !args.no_ui {
79        commands
80            .spawn(Node {
81                width: percent(100),
82                align_items: AlignItems::Center,
83                justify_content: JustifyContent::Center,
84                ..default()
85            })
86            .with_children(|commands| {
87                commands
88                    .spawn(Node {
89                        width: px(1000),
90                        ..Default::default()
91                    })
92                    .with_child((Text(text_string.clone()), text_font.clone(), text_block));
93            });
94    }
95
96    if !args.no_text2d {
97        commands.spawn((
98            Text2d::new(text_string),
99            text_font.clone(),
100            TextColor(RED.into()),
101            bevy::sprite::Anchor::CENTER,
102            TextBounds::new_horizontal(1000.),
103            text_block,
104        ));
105    }
106}
```

Additional examples can be found in:  

*   [examples/stress\_tests/many\_buttons.rs](../../../src/many_buttons/many_buttons.rs.html#309-316)
*   [examples/window/multiple\_windows.rs](../../../src/multiple_windows/multiple_windows.rs.html#60)
*   [examples/3d/spherical\_area\_lights.rs](../../../src/spherical_area_lights/spherical_area_lights.rs.html#60-64)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../../src/ui_texture_atlas/ui_texture_atlas.rs.html#60-64)
*   [examples/animation/animated\_ui.rs](../../../src/animated_ui/animated_ui.rs.html#162-174)
*   [examples/ui/layout/ghost\_nodes.rs](../../../src/ghost_nodes/ghost_nodes.rs.html#32-35)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../../src/overflow_debug/overflow_debug.rs.html#99-102)
*   [examples/ecs/error\_handling.rs](../../../src/error_handling/error_handling.rs.html#114-118)
*   [examples/ui/images/ui\_texture\_slice.rs](../../../src/ui_texture_slice/ui_texture_slice.rs.html#84-92)
*   [examples/ui/layout/size\_constraints.rs](../../../src/size_constraints/size_constraints.rs.html#179)
*   [examples/picking/dragdrop\_picking.rs](../../../src/dragdrop_picking/dragdrop_picking.rs.html#61-65)
*   [examples/ecs/iter\_combinations.rs](../../../src/iter_combinations/iter_combinations.rs.html#110-115)
*   [examples/ui/text/text\_wrap\_debug.rs](../../../src/text_wrap_debug/text_wrap_debug.rs.html#116-121)
*   [examples/window/multi\_window\_text.rs](../../../src/multi_window_text/multi_window_text.rs.html#69-73)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../../src/overflow_clip_margin/overflow_clip_margin.rs.html#53)
*   [examples/gizmos/light\_gizmos.rs](../../../src/light_gizmos/light_gizmos.rs.html#146)
*   [examples/ui/navigation/directional\_navigation.rs](../../../src/directional_navigation/directional_navigation.rs.html#241-247)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#96)
*   [examples/ui/text/text.rs](../../../src/text/text.rs.html#69-82)
*   [examples/usage/debug\_frustum\_culling.rs](../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#242-247)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#220-227)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#410)

### impl<'a> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#412)

#### pub fn [with\_related](#method.with_related)<R>( &mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Spawns a entity related to this entity (with the `R` relationship) by taking a bundle

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/ecs/relationships.rs ([line 60](../../../src/relationships/relationships.rs.html#60))

```rust
47    fn spawning_entities_with_relationships(mut commands: Commands) {
48        // Calling .id() after spawning an entity will return the `Entity` identifier of the spawned entity,
49        // even though the entity itself is not yet instantiated in the world.
50        // This works because Commands will reserve the entity ID before actually spawning the entity,
51        // through the use of atomic counters.
52        let alice = commands.spawn(Name::new("Alice")).id();
53        // Relations are just components, so we can add them into the bundle that we're spawning.
54        let bob = commands.spawn((Name::new("Bob"), Targeting(alice))).id();
55
56        // The `with_related` and `with_related_entities` helper methods on `EntityCommands` can be used to add relations in a more ergonomic way.
57        let charlie = commands
58            .spawn((Name::new("Charlie"), Targeting(bob)))
59            // The `with_related` method will spawn a bundle with `Targeting` relationship
60            .with_related::<Targeting>(Name::new("James"))
61            // The `with_related_entities` method will automatically add the `Targeting` component to any entities spawned within the closure,
62            // targeting the entity that we're calling `with_related` on.
63            .with_related_entities::<Targeting>(|related_spawner_commands| {
64                // We could spawn multiple entities here, and they would all target `charlie`.
65                related_spawner_commands.spawn(Name::new("Devon"));
66            })
67            .id();
68
69        // Simply inserting the `Targeting` component will automatically create and update the `TargetedBy` component on the target entity.
70        // We can do this at any point; not just when the entity is spawned.
71        commands.entity(alice).insert(Targeting(charlie));
72    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#419-422)

#### pub fn [with\_related\_entities](#method.with_related_entities)<R>( &mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [RelatedSpawnerCommands](../relationship/struct.RelatedSpawnerCommands.html "struct bevy::ecs::relationship::RelatedSpawnerCommands")<'\_, R>), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Spawns entities related to this entity (with the `R` relationship) by taking a function that operates on a [`RelatedSpawner`](../relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner").

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/ecs/relationships.rs ([lines 63-66](../../../src/relationships/relationships.rs.html#63-66))

```rust
47    fn spawning_entities_with_relationships(mut commands: Commands) {
48        // Calling .id() after spawning an entity will return the `Entity` identifier of the spawned entity,
49        // even though the entity itself is not yet instantiated in the world.
50        // This works because Commands will reserve the entity ID before actually spawning the entity,
51        // through the use of atomic counters.
52        let alice = commands.spawn(Name::new("Alice")).id();
53        // Relations are just components, so we can add them into the bundle that we're spawning.
54        let bob = commands.spawn((Name::new("Bob"), Targeting(alice))).id();
55
56        // The `with_related` and `with_related_entities` helper methods on `EntityCommands` can be used to add relations in a more ergonomic way.
57        let charlie = commands
58            .spawn((Name::new("Charlie"), Targeting(bob)))
59            // The `with_related` method will spawn a bundle with `Targeting` relationship
60            .with_related::<Targeting>(Name::new("James"))
61            // The `with_related_entities` method will automatically add the `Targeting` component to any entities spawned within the closure,
62            // targeting the entity that we're calling `with_related` on.
63            .with_related_entities::<Targeting>(|related_spawner_commands| {
64                // We could spawn multiple entities here, and they would all target `charlie`.
65                related_spawner_commands.spawn(Name::new("Devon"));
66            })
67            .id();
68
69        // Simply inserting the `Targeting` component will automatically create and update the `TargetedBy` component on the target entity.
70        // We can do this at any point; not just when the entity is spawned.
71        commands.entity(alice).insert(Targeting(charlie));
72    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#431)

#### pub fn [add\_related](#method.add_related)<R>(&mut self, related: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\]) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Relates the given entities to this entity with the relation `R`.

See [`add_one_related`](../../prelude/struct.EntityCommands.html#method.add_one_related "method bevy::prelude::EntityCommands::add_one_related") if you want relate only one entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#440)

#### pub fn [detach\_all\_related](#method.detach_all_related)<R>(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Removes the relation `R` between this entity and all its related entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#451-454)

#### pub fn [insert\_related](#method.insert_related)<R>( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), related: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), <<R as [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship")\>::[RelationshipTarget](../relationship/trait.Relationship.html#associatedtype.RelationshipTarget "type bevy::ecs::relationship::Relationship::RelationshipTarget") as [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget")\>::[Collection](../../prelude/trait.RelationshipTarget.html#associatedtype.Collection "type bevy::prelude::RelationshipTarget::Collection"): [OrderedRelationshipSourceCollection](../relationship/trait.OrderedRelationshipSourceCollection.html "trait bevy::ecs::relationship::OrderedRelationshipSourceCollection"),

Relates the given entities to this entity with the relation `R`, starting at this particular index.

If the `related` has duplicates, a related entity will take the index of its last occurrence in `related`. If the indices go out of bounds, they will be clamped into bounds. This will not re-order existing related entities unless they are in `related`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#466)

#### pub fn [add\_one\_related](#method.add_one_related)<R>(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Relates the given entity to this with the relation `R`.

See [`add_related`](../../prelude/struct.EntityCommands.html#method.add_related "method bevy::prelude::EntityCommands::add_related") if you want to relate more than one entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#471)

#### pub fn [remove\_related](#method.remove_related)<R>( &mut self, related: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Removes the relation `R` between this entity and the given entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#480)

#### pub fn [replace\_related](#method.replace_related)<R>( &mut self, related: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Replaces all the related entities with the given set of new related entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#498-503)

#### pub fn [replace\_related\_with\_difference](#method.replace_related_with_difference)<R>( &mut self, entities\_to\_unrelate: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], entities\_to\_relate: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], newly\_related\_entities: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Replaces all the related entities with a new set of entities.

##### Warning

Failing to maintain the functions invariants may lead to erratic engine behavior including random crashes. Refer to [`EntityWorldMut::replace_related_with_difference`](../../prelude/struct.EntityWorldMut.html#method.replace_related_with_difference "method bevy::prelude::EntityWorldMut::replace_related_with_difference") for a list of these invariants.

##### Panics

Panics when debug assertions are enable, an invariant is are broken and the command is executed.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#519)

#### pub fn [despawn\_related](#method.despawn_related)<S>(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where S: [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"),

Despawns entities that relate to this one via the given [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"). This entity will not be despawned.

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/diagnostics/log\_diagnostics.rs ([line 197](../../../src/log_diagnostics/log_diagnostics.rs.html#197))

```rust
180fn update_commands(
181    mut commands: Commands,
182    log_commands: Single<Entity, With<LogDiagnosticsCommands>>,
183    status: Res<LogDiagnosticsStatus>,
184    filters: Res<LogDiagnosticsFilters>,
185) {
186    let enabled = *status == LogDiagnosticsStatus::Enabled;
187    let alpha = if enabled { 1. } else { 0.25 };
188    let enabled_color = |enabled| {
189        if enabled {
190            Color::from(palettes::tailwind::GREEN_400)
191        } else {
192            Color::from(palettes::tailwind::RED_400)
193        }
194    };
195    commands
196        .entity(*log_commands)
197        .despawn_related::<Children>()
198        .insert(children![
199            (
200                Node {
201                    flex_direction: FlexDirection::Row,
202                    column_gap: px(5),
203                    ..default()
204                },
205                children![
206                    Text::new("[Q] Toggle filtering:"),
207                    (
208                        Text::new(format!("{:?}", *status)),
209                        TextColor(enabled_color(enabled))
210                    )
211                ]
212            ),
213            (
214                Node {
215                    flex_direction: FlexDirection::Row,
216                    column_gap: px(5),
217                    ..default()
218                },
219                children![
220                    (
221                        Text::new("[1] Frame times:"),
222                        TextColor(Color::WHITE.with_alpha(alpha))
223                    ),
224                    (
225                        Text::new(format!("{:?}", filters.frame_time)),
226                        TextColor(enabled_color(filters.frame_time).with_alpha(alpha))
227                    )
228                ]
229            ),
230            (
231                Node {
232                    flex_direction: FlexDirection::Row,
233                    column_gap: px(5),
234                    ..default()
235                },
236                children![
237                    (
238                        Text::new("[2] Entity count:"),
239                        TextColor(Color::WHITE.with_alpha(alpha))
240                    ),
241                    (
242                        Text::new(format!("{:?}", filters.entity_count)),
243                        TextColor(enabled_color(filters.entity_count).with_alpha(alpha))
244                    )
245                ]
246            ),
247            (
248                Node {
249                    flex_direction: FlexDirection::Row,
250                    column_gap: px(5),
251                    ..default()
252                },
253                children![
254                    (
255                        Text::new("[3] System info:"),
256                        TextColor(Color::WHITE.with_alpha(alpha))
257                    ),
258                    (
259                        Text::new(format!("{:?}", filters.system_info)),
260                        TextColor(enabled_color(filters.system_info).with_alpha(alpha))
261                    )
262                ]
263            ),
264            (
265                Node {
266                    flex_direction: FlexDirection::Row,
267                    column_gap: px(5),
268                    ..default()
269                },
270                children![
271                    (
272                        Text::new("[4] Render diagnostics:"),
273                        TextColor(Color::WHITE.with_alpha(alpha))
274                    ),
275                    (
276                        Text::new("Private"),
277                        TextColor(enabled_color(false).with_alpha(alpha))
278                    )
279                ]
280            ),
281        ]);
282}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#529)

#### pub fn [despawn\_children](#method.despawn_children)(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Despawns the children of this entity. This entity will not be despawned.

This is a specialization of [`despawn_related`](../../prelude/struct.EntityCommands.html#method.despawn_related "method bevy::prelude::EntityCommands::despawn_related"), a more general method for despawning via relationships.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#540-543)

#### pub fn [insert\_recursive](#method.insert_recursive)<S>( &mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where S: [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"),

Inserts a component or bundle of components into the entity and all related entities, traversing the relationship tracked in `S` in a breadth-first manner.

##### Warning

This method should only be called on relationships that form a tree-like structure. Any cycles will cause this method to loop infinitely.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#556)

#### pub fn [remove\_recursive](#method.remove_recursive)<S, B>(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where S: [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes a component or bundle of components of type `B` from the entity and all related entities, traversing the relationship tracked in `S` in a breadth-first manner.

##### Warning

This method should only be called on relationships that form a tree-like structure. Any cycles will cause this method to loop infinitely.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1309)

### impl<'a> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1324)

#### pub fn [id](#method.id)(&self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Returns the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") id of the entity.

##### Example

```rust
fn my_system(mut commands: Commands) {
    let entity_id = commands.spawn_empty().id();
}
```

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/scrollbars.rs ([line 21](../../../src/scrollbars/scrollbars.rs.html#21))

```rust
20fn setup_view_root(mut commands: Commands) {
21    let camera = commands.spawn((Camera::default(), Camera2d)).id();
22
23    commands.spawn((
24        Node {
25            display: Display::Flex,
26            flex_direction: FlexDirection::Column,
27            position_type: PositionType::Absolute,
28            left: px(0),
29            top: px(0),
30            right: px(0),
31            bottom: px(0),
32            padding: UiRect::all(px(3)),
33            row_gap: px(6),
34            ..Default::default()
35        },
36        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
37        UiTargetCamera(camera),
38        TabGroup::default(),
39        Children::spawn((Spawn(Text::new("Scrolling")), Spawn(scroll_area_demo()))),
40    ));
41}
```

Hide additional examples

examples/3d/ssr.rs ([line 303](../../../src/ssr/ssr.rs.html#303))

```rust
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
```

examples/state/custom\_transitions.rs ([line 277](../../../src/custom_transitions/custom_transitions.rs.html#277))

```rust
244fn setup_menu(mut commands: Commands) {
245    let button_entity = commands
246        .spawn((
247            Node {
248                // center button
249                width: percent(100),
250                height: percent(100),
251                justify_content: JustifyContent::Center,
252                align_items: AlignItems::Center,
253                ..default()
254            },
255            children![(
256                Button,
257                Node {
258                    width: px(150),
259                    height: px(65),
260                    // horizontally center child text
261                    justify_content: JustifyContent::Center,
262                    // vertically center child text
263                    align_items: AlignItems::Center,
264                    ..default()
265                },
266                BackgroundColor(NORMAL_BUTTON),
267                children![(
268                    Text::new("Play"),
269                    TextFont {
270                        font_size: FontSize::Px(33.0),
271                        ..default()
272                    },
273                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
274                )]
275            )],
276        ))
277        .id();
278    commands.insert_resource(MenuData { button_entity });
279}
```

examples/state/states.rs ([line 90](../../../src/states/states.rs.html#90))

```rust
57fn setup_menu(mut commands: Commands) {
58    let button_entity = commands
59        .spawn((
60            Node {
61                // center button
62                width: percent(100),
63                height: percent(100),
64                justify_content: JustifyContent::Center,
65                align_items: AlignItems::Center,
66                ..default()
67            },
68            children![(
69                Button,
70                Node {
71                    width: px(150),
72                    height: px(65),
73                    // horizontally center child text
74                    justify_content: JustifyContent::Center,
75                    // vertically center child text
76                    align_items: AlignItems::Center,
77                    ..default()
78                },
79                BackgroundColor(NORMAL_BUTTON),
80                children![(
81                    Text::new("Play"),
82                    TextFont {
83                        font_size: FontSize::Px(33.0),
84                        ..default()
85                    },
86                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
87                )],
88            )],
89        ))
90        .id();
91    commands.insert_resource(MenuData { button_entity });
92}
```

examples/state/sub\_states.rs ([line 190](../../../src/sub_states/sub_states.rs.html#190))

```rust
157    pub fn setup_menu(mut commands: Commands) {
158        let button_entity = commands
159            .spawn((
160                Node {
161                    // center button
162                    width: percent(100),
163                    height: percent(100),
164                    justify_content: JustifyContent::Center,
165                    align_items: AlignItems::Center,
166                    ..default()
167                },
168                children![(
169                    Button,
170                    Node {
171                        width: px(150),
172                        height: px(65),
173                        // horizontally center child text
174                        justify_content: JustifyContent::Center,
175                        // vertically center child text
176                        align_items: AlignItems::Center,
177                        ..default()
178                    },
179                    BackgroundColor(NORMAL_BUTTON),
180                    children![(
181                        Text::new("Play"),
182                        TextFont {
183                            font_size: FontSize::Px(33.0),
184                            ..default()
185                        },
186                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
187                    )]
188                )],
189            ))
190            .id();
191        commands.insert_resource(MenuData { button_entity });
192    }
```

examples/ecs/hierarchy.rs ([line 42](../../../src/hierarchy/hierarchy.rs.html#42))

```rust
19fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
20    commands.spawn(Camera2d);
21    let texture = asset_server.load("branding/icon.png");
22
23    // Spawn a root entity with no parent
24    let parent = commands
25        .spawn((
26            Sprite::from_image(texture.clone()),
27            Transform::from_scale(Vec3::splat(0.75)),
28        ))
29        // With that entity as a parent, run a lambda that spawns its children
30        .with_children(|parent| {
31            // parent is a ChildSpawnerCommands, which has a similar API to Commands
32            parent.spawn((
33                Transform::from_xyz(250.0, 0.0, 0.0).with_scale(Vec3::splat(0.75)),
34                Sprite {
35                    image: texture.clone(),
36                    color: BLUE.into(),
37                    ..default()
38                },
39            ));
40        })
41        // Store parent entity for next sections
42        .id();
43
44    // Another way is to use the add_child function to add children after the parent
45    // entity has already been spawned.
46    let child = commands
47        .spawn((
48            Sprite {
49                image: texture,
50                color: LIME.into(),
51                ..default()
52            },
53            Transform::from_xyz(0.0, 250.0, 0.0).with_scale(Vec3::splat(0.75)),
54        ))
55        .id();
56
57    // Add child to the parent.
58    commands.entity(parent).add_child(child);
59}
```

Additional examples can be found in:  

*   [examples/ui/widgets/standard\_widgets.rs](../../../src/standard_widgets/standard_widgets.rs.html#865)
*   [examples/showcase/contributors.rs](../../../src/contributors/contributors.rs.html#131)
*   [examples/window/multiple\_windows.rs](../../../src/multiple_windows/multiple_windows.rs.html#29)
*   [examples/ecs/observers.rs](../../../src/observers/observers.rs.html#134)
*   [examples/ecs/relationships.rs](../../../src/relationships/relationships.rs.html#52)
*   [examples/3d/occlusion\_culling.rs](../../../src/occlusion_culling/occlusion_culling.rs.html#278)
*   [examples/animation/animated\_ui.rs](../../../src/animated_ui/animated_ui.rs.html#161)
*   [examples/animation/eased\_motion.rs](../../../src/eased_motion/eased_motion.rs.html#48)
*   [examples/ui/text/text\_input.rs](../../../src/text_input/text_input.rs.html#59)
*   [examples/ui/widgets/viewport\_node.rs](../../../src/viewport_node/viewport_node.rs.html#57)
*   [examples/ui/text/ime\_support.rs](../../../src/ime_support/ime_support.rs.html#41)
*   [examples/window/monitor\_info.rs](../../../src/monitor_info/monitor_info.rs.html#50)
*   [examples/async\_tasks/async\_compute.rs](../../../src/async_compute/async_compute.rs.html#74)
*   [examples/animation/animation\_graph.rs](../../../src/animation_graph/animation_graph.rs.html#288)
*   [examples/3d/irradiance\_volumes.rs](../../../src/irradiance_volumes/irradiance_volumes.rs.html#590)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../../src/many_morph_targets/many_morph_targets.rs.html#360)
*   [examples/state/computed\_states.rs](../../../src/computed_states/computed_states.rs.html#399)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../../src/transform_hierarchy/transform_hierarchy.rs.html#379)
*   [examples/stress\_tests/many\_buttons.rs](../../../src/many_buttons/many_buttons.rs.html#355)
*   [examples/ui/text/text\_wrap\_debug.rs](../../../src/text_wrap_debug/text_wrap_debug.rs.html#63)
*   [examples/showcase/alien\_cake\_addict.rs](../../../src/alien_cake_addict/alien_cake_addict.rs.html#172)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#754)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#157)
*   [examples/ui/ui\_target\_camera.rs](../../../src/ui_target_camera/ui_target_camera.rs.html#45)
*   [examples/window/multi\_window\_text.rs](../../../src/multi_window_text/multi_window_text.rs.html#42)
*   [examples/ui/widgets/vertical\_slider.rs](../../../src/vertical_slider/vertical_slider.rs.html#84)
*   [examples/3d/split\_screen.rs](../../../src/split_screen/split_screen.rs.html#82)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#183)
*   [examples/ui/render\_ui\_to\_texture.rs](../../../src/render_ui_to_texture/render_ui_to_texture.rs.html#74)
*   [examples/ui/images/image\_node\_resizing.rs](../../../src/image_node_resizing/image_node_resizing.rs.html#82)
*   [examples/ui/layout/display\_and\_visibility.rs](../../../src/display_and_visibility/display_and_visibility.rs.html#244)
*   [examples/ui/navigation/directional\_navigation.rs](../../../src/directional_navigation/directional_navigation.rs.html#124)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#168)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#220)
*   [examples/3d/contact\_shadows.rs](../../../src/contact_shadows/contact_shadows.rs.html#147)
*   [examples/animation/custom\_skinned\_mesh.rs](../../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#161)
*   [examples/usage/debug\_frustum\_culling.rs](../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#117)
*   [examples/3d/blend\_modes.rs](../../../src/blend_modes/blend_modes.rs.html#51)
*   [examples/animation/animated\_transform.rs](../../../src/animated_transform/animated_transform.rs.html#153)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#174)
*   [examples/ui/text/text\_debug.rs](../../../src/text_debug/text_debug.rs.html#45)
*   [examples/ui/styling/gradients.rs](../../../src/gradients/gradients.rs.html#271)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1331)

#### pub fn [reborrow](#method.reborrow)(&mut self) -> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Returns an [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") with a smaller lifetime.

This is useful if you have `&mut EntityCommands` but you need `EntityCommands`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1381)

#### pub fn [entry](#method.entry)<T>(&mut self) -> [EntityEntryCommands](../system/struct.EntityEntryCommands.html "struct bevy::ecs::system::EntityEntryCommands")<'\_, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Get an [`EntityEntryCommands`](../system/struct.EntityEntryCommands.html "struct bevy::ecs::system::EntityEntryCommands") for the [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") `T`, allowing you to modify it or insert it if it isn’t already present.

See also [`insert_if_new`](../../prelude/struct.EntityCommands.html#method.insert_if_new "method bevy::prelude::EntityCommands::insert_if_new"), which lets you insert a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") without overwriting it.

##### Example

```rust
#[derive(Component)]
struct Level(u32);


#[derive(Component, Default)]
struct Mana {
    max: u32,
    current: u32,
}

fn level_up_system(mut commands: Commands, player: Res<PlayerEntity>) {
    // If a component already exists then modify it, otherwise insert a default value
    commands
        .entity(player.entity)
        .entry::<Level>()
        .and_modify(|mut lvl| lvl.0 += 1)
        .or_insert(Level(0));

    // Add a default value if none exists, and then modify the existing or new value
    commands
        .entity(player.entity)
        .entry::<Mana>()
        .or_default()
        .and_modify(|mut mana| {
            mana.max += 10;
            mana.current = mana.max;
    });
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1435)

#### pub fn [insert](#method.insert)(&mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity.

This will overwrite any previous value(s) of the same component type. See [`EntityCommands::insert_if_new`](../../prelude/struct.EntityCommands.html#method.insert_if_new "method bevy::prelude::EntityCommands::insert_if_new") to keep the old value instead.

##### Example

```rust
#[derive(Component)]
struct Health(u32);
#[derive(Component)]
struct Strength(u32);
#[derive(Component)]
struct Defense(u32);

#[derive(Bundle)]
struct CombatBundle {
    health: Health,
    strength: Strength,
}

fn add_combat_stats_system(mut commands: Commands, player: Res<PlayerEntity>) {
    commands
        .entity(player.entity)
        // You can insert individual components:
        .insert(Defense(10))
        // You can also insert pre-defined bundles of components:
        .insert(CombatBundle {
            health: Health(100),
            strength: Strength(40),
        })
        // You can also insert tuples of components and bundles.
        // This is equivalent to the calls above:
        .insert((
            Defense(10),
            CombatBundle {
                health: Health(100),
                strength: Strength(40),
            },
        ));
}
```

##### [Examples found in repository](#scraped-examples-8)[?](../../../scrape-examples-help.html)

examples/3d/mixed\_lighting.rs ([line 160](../../../src/mixed_lighting/mixed_lighting.rs.html#160))

```rust
157fn spawn_camera(commands: &mut Commands) {
158    commands
159        .spawn(Camera3d::default())
160        .insert(Transform::from_xyz(-0.7, 0.7, 1.0).looking_at(vec3(0.0, 0.3, 0.0), Vec3::Y));
161}
162
163/// Spawns the scene.
164///
165/// The scene is loaded from a glTF file.
166fn spawn_scene(commands: &mut Commands, asset_server: &AssetServer) {
167    commands
168        .spawn(WorldAssetRoot(
169            asset_server.load(
170                GltfAssetLabel::Scene(0)
171                    .from_asset("models/MixedLightingExample/MixedLightingExample.gltf"),
172            ),
173        ))
174        .observe(
175            |_: On<WorldInstanceReady>,
176             mut lighting_mode_changed_writer: MessageWriter<LightingModeChanged>| {
177                // When the scene loads, send a `LightingModeChanged` event so
178                // that we set up the lightmaps.
179                lighting_mode_changed_writer.write(LightingModeChanged);
180            },
181        );
182}
183
184/// Spawns the buttons that allow the user to change the lighting mode.
185fn spawn_buttons(commands: &mut Commands) {
186    commands.spawn((
187        widgets::main_ui_node(),
188        children![widgets::option_buttons(
189            "Lighting",
190            &[
191                (LightingMode::Baked, "Baked"),
192                (LightingMode::MixedDirect, "Mixed (Direct)"),
193                (LightingMode::MixedIndirect, "Mixed (Indirect)"),
194                (LightingMode::RealTime, "Real-Time"),
195            ],
196        )],
197    ));
198}
199
200/// Spawns the help text at the top of the window.
201fn spawn_help_text(commands: &mut Commands, app_status: &AppStatus) {
202    commands.spawn((
203        create_help_text(app_status),
204        Node {
205            position_type: PositionType::Absolute,
206            top: px(12),
207            left: px(12),
208            ..default()
209        },
210        HelpText,
211    ));
212}
213
214/// Adds lightmaps to and/or removes lightmaps from objects in the scene when
215/// the lighting mode changes.
216///
217/// This is also called right after the scene loads in order to set up the
218/// lightmaps.
219fn update_lightmaps(
220    mut commands: Commands,
221    asset_server: Res<AssetServer>,
222    mut materials: ResMut<Assets<StandardMaterial>>,
223    meshes: Query<(Entity, &GltfMeshName, &MeshMaterial3d<StandardMaterial>), With<Mesh3d>>,
224    mut lighting_mode_changed_reader: MessageReader<LightingModeChanged>,
225    app_status: Res<AppStatus>,
226) {
227    // Only run if the lighting mode changed. (Note that a change event is fired
228    // when the scene first loads.)
229    if lighting_mode_changed_reader.read().next().is_none() {
230        return;
231    }
232
233    // Select the lightmap to use, based on the lighting mode.
234    let lightmap: Option<Handle<Image>> = match app_status.lighting_mode {
235        LightingMode::Baked => {
236            Some(asset_server.load("lightmaps/MixedLightingExample-Baked.zstd.ktx2"))
237        }
238        LightingMode::MixedDirect => {
239            Some(asset_server.load("lightmaps/MixedLightingExample-MixedDirect.zstd.ktx2"))
240        }
241        LightingMode::MixedIndirect => {
242            Some(asset_server.load("lightmaps/MixedLightingExample-MixedIndirect.zstd.ktx2"))
243        }
244        LightingMode::RealTime => None,
245    };
246
247    'outer: for (entity, name, material) in &meshes {
248        // Add lightmaps to or remove lightmaps from the scenery objects in the
249        // scene (all objects but the sphere).
250        //
251        // Note that doing a linear search through the `LIGHTMAPS` array is
252        // inefficient, but we do it anyway in this example to improve clarity.
253        for (lightmap_name, uv_rect) in LIGHTMAPS {
254            if &**name != lightmap_name {
255                continue;
256            }
257
258            // Lightmap exposure defaults to zero, so we need to set it.
259            if let Some(ref mut material) = materials.get_mut(material) {
260                material.lightmap_exposure = LIGHTMAP_EXPOSURE;
261            }
262
263            // Add or remove the lightmap.
264            match lightmap {
265                Some(ref lightmap) => {
266                    commands.entity(entity).insert(Lightmap {
267                        image: (*lightmap).clone(),
268                        uv_rect,
269                        bicubic_sampling: false,
270                    });
271                }
272                None => {
273                    commands.entity(entity).remove::<Lightmap>();
274                }
275            }
276            continue 'outer;
277        }
278
279        // Add lightmaps to or remove lightmaps from the sphere.
280        if &**name == "Sphere" {
281            // Lightmap exposure defaults to zero, so we need to set it.
282            if let Some(ref mut material) = materials.get_mut(material) {
283                material.lightmap_exposure = LIGHTMAP_EXPOSURE;
284            }
285
286            // Add or remove the lightmap from the sphere. We only apply the
287            // lightmap in fully-baked mode.
288            match (&lightmap, app_status.lighting_mode) {
289                (Some(lightmap), LightingMode::Baked) => {
290                    commands.entity(entity).insert(Lightmap {
291                        image: (*lightmap).clone(),
292                        uv_rect: SPHERE_UV_RECT,
293                        bicubic_sampling: false,
294                    });
295                }
296                _ => {
297                    commands.entity(entity).remove::<Lightmap>();
298                }
299            }
300        }
301    }
302}
303
304/// Converts a uv rectangle from the OpenGL coordinate system (origin in the
305/// lower left) to the Vulkan coordinate system (origin in the upper left) that
306/// Bevy uses.
307///
308/// For this particular example, the baking tool happened to use the OpenGL
309/// coordinate system, so it was more convenient to do the conversion at compile
310/// time than to pre-calculate and hard-code the values.
311const fn uv_rect_opengl(gl_min: Vec2, size: Vec2) -> Rect {
312    let min = vec2(gl_min.x, 1.0 - gl_min.y - size.y);
313    Rect {
314        min,
315        max: vec2(min.x + size.x, min.y + size.y),
316    }
317}
318
319/// Ensures that clicking on the scene to move the sphere doesn't result in a
320/// hit on the sphere itself.
321fn make_sphere_nonpickable(
322    mut commands: Commands,
323    mut query: Query<(Entity, &Name), (With<Mesh3d>, Without<Pickable>)>,
324) {
325    for (sphere, name) in &mut query {
326        if &**name == "Sphere" {
327            commands.entity(sphere).insert(Pickable::IGNORE);
328        }
329    }
330}
331
332/// Updates the directional light settings as necessary when the lighting mode
333/// changes.
334fn update_directional_light(
335    mut lights: Query<&mut DirectionalLight>,
336    mut lighting_mode_changed_reader: MessageReader<LightingModeChanged>,
337    app_status: Res<AppStatus>,
338) {
339    // Only run if the lighting mode changed. (Note that a change event is fired
340    // when the scene first loads.)
341    if lighting_mode_changed_reader.read().next().is_none() {
342        return;
343    }
344
345    // Real-time direct light is used on the scenery if we're using mixed
346    // indirect or real-time mode.
347    let scenery_is_lit_in_real_time = matches!(
348        app_status.lighting_mode,
349        LightingMode::MixedIndirect | LightingMode::RealTime
350    );
351
352    for mut light in &mut lights {
353        light.affects_lightmapped_mesh_diffuse = scenery_is_lit_in_real_time;
354        // Don't bother enabling shadows if they won't show up on the scenery.
355        light.shadow_maps_enabled = scenery_is_lit_in_real_time;
356    }
357}
358
359/// Updates the state of the selection widgets at the bottom of the window when
360/// the lighting mode changes.
361fn update_radio_buttons(
362    mut widgets: Query<
363        (
364            Entity,
365            Option<&mut BackgroundColor>,
366            Has<Text>,
367            &WidgetClickSender<LightingMode>,
368        ),
369        Or<(With<RadioButton>, With<RadioButtonText>)>,
370    >,
371    app_status: Res<AppStatus>,
372    mut writer: TextUiWriter,
373) {
374    for (entity, image, has_text, sender) in &mut widgets {
375        let selected = **sender == app_status.lighting_mode;
376
377        if let Some(mut bg_color) = image {
378            widgets::update_ui_radio_button(&mut bg_color, selected);
379        }
380        if has_text {
381            widgets::update_ui_radio_button_text(entity, &mut writer, selected);
382        }
383    }
384}
385
386/// Handles clicks on the widgets at the bottom of the screen and fires
387/// [`LightingModeChanged`] events.
388fn handle_lighting_mode_change(
389    mut widget_click_event_reader: MessageReader<WidgetClickEvent<LightingMode>>,
390    mut lighting_mode_changed_writer: MessageWriter<LightingModeChanged>,
391    mut app_status: ResMut<AppStatus>,
392) {
393    for event in widget_click_event_reader.read() {
394        app_status.lighting_mode = **event;
395        lighting_mode_changed_writer.write(LightingModeChanged);
396    }
397}
398
399/// Moves the sphere to its original position when the user selects the baked
400/// lighting mode.
401///
402/// As the light from the sphere is precomputed and depends on the sphere's
403/// original position, the sphere must be placed there in order for the lighting
404/// to be correct.
405fn reset_sphere_position(
406    mut objects: Query<(&Name, &mut Transform)>,
407    mut lighting_mode_changed_reader: MessageReader<LightingModeChanged>,
408    app_status: Res<AppStatus>,
409) {
410    // Only run if the lighting mode changed and if the lighting mode is
411    // `LightingMode::Baked`. (Note that a change event is fired when the scene
412    // first loads.)
413    if lighting_mode_changed_reader.read().next().is_none()
414        || app_status.lighting_mode != LightingMode::Baked
415    {
416        return;
417    }
418
419    for (name, mut transform) in &mut objects {
420        if &**name == "Sphere" {
421            transform.translation = INITIAL_SPHERE_POSITION;
422            break;
423        }
424    }
425}
426
427/// Updates the position of the sphere when the user clicks on a spot in the
428/// scene.
429///
430/// Note that the position of the sphere is locked in baked lighting mode.
431fn move_sphere(
432    mouse_button_input: Res<ButtonInput<MouseButton>>,
433    pointers: Query<&PointerInteraction>,
434    mut meshes: Query<(&GltfMeshName, &ChildOf), With<Mesh3d>>,
435    mut transforms: Query<&mut Transform>,
436    app_status: Res<AppStatus>,
437) {
438    // Only run when the left button is clicked and we're not in baked lighting
439    // mode.
440    if app_status.lighting_mode == LightingMode::Baked
441        || !mouse_button_input.pressed(MouseButton::Left)
442    {
443        return;
444    }
445
446    // Find the sphere.
447    let Some(child_of) = meshes
448        .iter_mut()
449        .filter_map(|(name, child_of)| {
450            if &**name == "Sphere" {
451                Some(child_of)
452            } else {
453                None
454            }
455        })
456        .next()
457    else {
458        return;
459    };
460
461    // Grab its transform.
462    let Ok(mut transform) = transforms.get_mut(child_of.parent()) else {
463        return;
464    };
465
466    // Set its transform to the appropriate position, as determined by the
467    // picking subsystem.
468    for interaction in pointers.iter() {
469        if let Some(&(
470            _,
471            HitData {
472                position: Some(position),
473                ..
474            },
475        )) = interaction.get_nearest_hit()
476        {
477            transform.translation = position + vec3(0.0, SPHERE_OFFSET, 0.0);
478        }
479    }
480}
481
482/// Changes the help text at the top of the screen when the lighting mode
483/// changes.
484fn adjust_help_text(
485    mut commands: Commands,
486    help_texts: Query<Entity, With<HelpText>>,
487    app_status: Res<AppStatus>,
488    mut lighting_mode_changed_reader: MessageReader<LightingModeChanged>,
489) {
490    if lighting_mode_changed_reader.read().next().is_none() {
491        return;
492    }
493
494    for help_text in &help_texts {
495        commands
496            .entity(help_text)
497            .insert(create_help_text(&app_status));
498    }
499}
```

Hide additional examples

examples/3d/clustered\_decals.rs ([line 212](../../../src/clustered_decals/clustered_decals.rs.html#212))

```rust
209fn spawn_camera(commands: &mut Commands) {
210    commands
211        .spawn(Camera3d::default())
212        .insert(Transform::from_xyz(0.0, 2.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
213        // Tag the camera with `Selection::Camera`.
214        .insert(Selection::Camera);
215}
216
217/// Spawns the actual clustered decals.
218fn spawn_decals(commands: &mut Commands, asset_server: &AssetServer) {
219    let base_color_texture = asset_server.load("branding/icon.png");
220
221    commands.spawn((
222        ClusteredDecal {
223            base_color_texture: Some(base_color_texture.clone()),
224            // Tint with red.
225            tag: 1,
226            ..ClusteredDecal::default()
227        },
228        calculate_initial_decal_transform(vec3(1.0, 3.0, 5.0), Vec3::ZERO, Vec2::splat(1.1)),
229        Selection::DecalA,
230    ));
231
232    commands.spawn((
233        ClusteredDecal {
234            base_color_texture: Some(base_color_texture.clone()),
235            // Tint with blue.
236            tag: 2,
237            ..ClusteredDecal::default()
238        },
239        calculate_initial_decal_transform(vec3(-2.0, -1.0, 4.0), Vec3::ZERO, Vec2::splat(2.0)),
240        Selection::DecalB,
241    ));
242}
243
244/// Spawns the buttons at the bottom of the screen.
245fn spawn_buttons(commands: &mut Commands) {
246    // Spawn the radio buttons that allow the user to select an object to
247    // control.
248    commands.spawn((
249        widgets::main_ui_node(),
250        children![widgets::option_buttons(
251            "Drag to Move",
252            &[
253                (Selection::Camera, "Camera"),
254                (Selection::DecalA, "Decal A"),
255                (Selection::DecalB, "Decal B"),
256            ],
257        )],
258    ));
259
260    // Spawn the drag buttons that allow the user to control the scale and roll
261    // of the selected object.
262    commands.spawn((
263        Node {
264            flex_direction: FlexDirection::Row,
265            position_type: PositionType::Absolute,
266            right: px(10),
267            bottom: px(10),
268            column_gap: px(6),
269            ..default()
270        },
271        children![
272            (drag_button("Scale"), DragMode::Scale),
273            (drag_button("Roll"), DragMode::Roll),
274        ],
275    ));
276}
277
278/// Spawns a button that the user can drag to change a parameter.
279fn drag_button(label: &str) -> impl Bundle {
280    (
281        Node {
282            border: BUTTON_BORDER,
283            justify_content: JustifyContent::Center,
284            align_items: AlignItems::Center,
285            padding: BUTTON_PADDING,
286            border_radius: BorderRadius::all(BUTTON_BORDER_RADIUS_SIZE),
287            ..default()
288        },
289        Button,
290        BackgroundColor(Color::BLACK),
291        BUTTON_BORDER_COLOR,
292        children![widgets::ui_text(label, Color::WHITE)],
293    )
294}
295
296/// Spawns the help text at the top of the screen.
297fn spawn_help_text(commands: &mut Commands, app_status: &AppStatus) {
298    commands.spawn((
299        Text::new(create_help_string(app_status)),
300        Node {
301            position_type: PositionType::Absolute,
302            top: px(12),
303            left: px(12),
304            ..default()
305        },
306        HelpText,
307    ));
308}
309
310/// Draws the outlines that show the bounds of the clustered decals.
311fn draw_gizmos(
312    mut gizmos: Gizmos,
313    decals: Query<(&GlobalTransform, &Selection), With<ClusteredDecal>>,
314) {
315    for (global_transform, selection) in &decals {
316        let color = match *selection {
317            Selection::Camera => continue,
318            Selection::DecalA => ORANGE_RED,
319            Selection::DecalB => LIME,
320        };
321
322        gizmos.primitive_3d(
323            &Cuboid {
324                // Since the clustered decal is a 1×1×1 cube in model space, its
325                // half-size is half of the scaling part of its transform.
326                half_size: global_transform.scale() * 0.5,
327            },
328            Isometry3d {
329                rotation: global_transform.rotation(),
330                translation: global_transform.translation_vec3a(),
331            },
332            color,
333        );
334    }
335}
336
337/// Calculates the initial transform of the clustered decal.
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
434
435/// Processes a drag event that scales the selected target.
436fn process_scale_input(
437    mut selections: Query<(&mut Transform, &Selection)>,
438    mouse_buttons: Res<ButtonInput<MouseButton>>,
439    mouse_motion: Res<AccumulatedMouseMotion>,
440    app_status: Res<AppStatus>,
441) {
442    // Only process drags when the scaling operation is selected.
443    if !mouse_buttons.pressed(MouseButton::Left) || app_status.drag_mode != DragMode::Scale {
444        return;
445    }
446
447    for (mut transform, selection) in &mut selections {
448        if app_status.selection == *selection {
449            transform.scale *= 1.0 + mouse_motion.delta.x * SCALE_SPEED;
450        }
451    }
452}
453
454/// Processes a drag event that rotates the selected target along its local Z
455/// axis.
456fn process_roll_input(
457    mut selections: Query<(&mut Transform, &Selection)>,
458    mouse_buttons: Res<ButtonInput<MouseButton>>,
459    mouse_motion: Res<AccumulatedMouseMotion>,
460    app_status: Res<AppStatus>,
461) {
462    // Only process drags when the rolling operation is selected.
463    if !mouse_buttons.pressed(MouseButton::Left) || app_status.drag_mode != DragMode::Roll {
464        return;
465    }
466
467    for (mut transform, selection) in &mut selections {
468        if app_status.selection != *selection {
469            continue;
470        }
471
472        let (yaw, pitch, mut roll) = transform.rotation.to_euler(EulerRot::YXZ);
473        roll += mouse_motion.delta.x * ROLL_SPEED;
474        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
475    }
476}
477
478/// Creates the help string at the top left of the screen.
479fn create_help_string(app_status: &AppStatus) -> String {
480    format!(
481        "Click and drag to {} {}",
482        app_status.drag_mode, app_status.selection
483    )
484}
485
486/// Changes the drag mode when the user hovers over the "Scale" and "Roll"
487/// buttons in the lower right.
488///
489/// If the user is hovering over no such button, this system changes the drag
490/// mode back to its default value of [`DragMode::Move`].
491fn switch_drag_mode(
492    mut commands: Commands,
493    mut interactions: Query<(&Interaction, &DragMode)>,
494    mut windows: Query<Entity, With<Window>>,
495    mouse_buttons: Res<ButtonInput<MouseButton>>,
496    mut app_status: ResMut<AppStatus>,
497) {
498    if mouse_buttons.pressed(MouseButton::Left) {
499        return;
500    }
501
502    for (interaction, drag_mode) in &mut interactions {
503        if *interaction != Interaction::Hovered {
504            continue;
505        }
506
507        app_status.drag_mode = *drag_mode;
508
509        // Set the cursor to provide the user with a nice visual hint.
510        for window in &mut windows {
511            commands
512                .entity(window)
513                .insert(CursorIcon::from(SystemCursorIcon::EwResize));
514        }
515        return;
516    }
517
518    app_status.drag_mode = DragMode::Move;
519
520    for window in &mut windows {
521        commands.entity(window).remove::<CursorIcon>();
522    }
523}
```

examples/3d/light\_textures.rs ([line 223](../../../src/light_textures/light_textures.rs.html#223))

```rust
220fn spawn_camera(commands: &mut Commands) {
221    commands
222        .spawn(Camera3d::default())
223        .insert(Transform::from_xyz(0.0, 2.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
224        // Tag the camera with `Selection::Camera`.
225        .insert(Selection::Camera);
226}
227
228fn spawn_light_textures(
229    commands: &mut Commands,
230    asset_server: &AssetServer,
231    meshes: &mut Assets<Mesh>,
232    materials: &mut Assets<StandardMaterial>,
233) {
234    commands.spawn((
235        SpotLight {
236            color: Color::srgb(1.0, 1.0, 0.8),
237            intensity: 10e6,
238            outer_angle: 0.25,
239            inner_angle: 0.25,
240            shadow_maps_enabled: true,
241            ..default()
242        },
243        Transform::from_translation(Vec3::new(6.0, 1.0, 2.0)).looking_at(Vec3::ZERO, Vec3::Y),
244        SpotLightTexture {
245            image: asset_server.load("lightmaps/torch_spotlight_texture.png"),
246        },
247        Visibility::Inherited,
248        Selection::SpotLight,
249    ));
250
251    commands.spawn((
252        Visibility::Hidden,
253        Transform::from_translation(Vec3::new(0.0, 1.8, 0.01)).with_scale(Vec3::splat(0.1)),
254        Selection::PointLight,
255        children![
256            WorldAssetRoot(
257                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Faces/faces.glb")),
258            ),
259            (
260                Mesh3d(meshes.add(Sphere::new(1.0))),
261                MeshMaterial3d(materials.add(StandardMaterial {
262                    emissive: Color::srgb(0.0, 0.0, 300.0).to_linear(),
263                    ..default()
264                })),
265            ),
266            (
267                PointLight {
268                    color: Color::srgb(0.0, 0.0, 1.0),
269                    intensity: 1e6,
270                    shadow_maps_enabled: true,
271                    ..default()
272                },
273                PointLightTexture {
274                    image: asset_server.load("lightmaps/faces_pointlight_texture_blurred.png"),
275                    cubemap_layout: CubemapLayout::CrossVertical,
276                },
277            )
278        ],
279    ));
280}
281
282/// Spawns the buttons at the bottom of the screen.
283fn spawn_buttons(commands: &mut Commands) {
284    // Spawn the radio buttons that allow the user to select an object to
285    // control.
286    commands.spawn((
287        widgets::main_ui_node(),
288        children![widgets::option_buttons(
289            "Drag to Move",
290            &[
291                (Selection::Camera, "Camera"),
292                (Selection::SpotLight, "Spotlight"),
293                (Selection::PointLight, "Point Light"),
294                (Selection::DirectionalLight, "Directional Light"),
295            ],
296        )],
297    ));
298
299    // Spawn the drag buttons that allow the user to control the scale and roll
300    // of the selected object.
301    commands.spawn((
302        Node {
303            flex_direction: FlexDirection::Row,
304            position_type: PositionType::Absolute,
305            right: px(10),
306            bottom: px(10),
307            column_gap: px(6),
308            ..default()
309        },
310        children![
311            widgets::option_buttons(
312                "",
313                &[
314                    (Visibility::Inherited, "Show"),
315                    (Visibility::Hidden, "Hide"),
316                ],
317            ),
318            (drag_button("Scale"), DragMode::Scale),
319            (drag_button("Roll"), DragMode::Roll),
320        ],
321    ));
322}
323
324/// Spawns a button that the user can drag to change a parameter.
325fn drag_button(label: &str) -> impl Bundle {
326    (
327        Node {
328            border: BUTTON_BORDER,
329            justify_content: JustifyContent::Center,
330            align_items: AlignItems::Center,
331            padding: BUTTON_PADDING,
332            border_radius: BorderRadius::all(BUTTON_BORDER_RADIUS_SIZE),
333            ..default()
334        },
335        Button,
336        BackgroundColor(Color::BLACK),
337        BUTTON_BORDER_COLOR,
338        children![widgets::ui_text(label, Color::WHITE),],
339    )
340}
341
342/// Spawns the help text at the top of the screen.
343fn spawn_help_text(commands: &mut Commands, app_status: &AppStatus) {
344    commands.spawn((
345        Text::new(create_help_string(app_status)),
346        Node {
347            position_type: PositionType::Absolute,
348            top: px(12),
349            left: px(12),
350            ..default()
351        },
352        HelpText,
353    ));
354}
355
356/// Draws the outlines that show the bounds of the spotlight.
357fn draw_gizmos(mut gizmos: Gizmos, spotlight: Query<(&GlobalTransform, &SpotLight, &Visibility)>) {
358    if let Ok((global_transform, spotlight, visibility)) = spotlight.single()
359        && visibility != Visibility::Hidden
360    {
361        gizmos.primitive_3d(
362            &Cone::new(7.0 * spotlight.outer_angle, 7.0),
363            Isometry3d {
364                rotation: global_transform.rotation() * Quat::from_rotation_x(FRAC_PI_2),
365                translation: global_transform.translation_vec3a() * 0.5,
366            },
367            YELLOW,
368        );
369    }
370}
371
372/// Rotates the cube a bit every frame.
373fn rotate_cube(mut meshes: Query<&mut Transform, With<Rotate>>) {
374    for mut transform in &mut meshes {
375        transform.rotate_y(CUBE_ROTATION_SPEED);
376    }
377}
378
379/// Hide shadows on all meshes except the main cube
380fn hide_shadows(
381    mut commands: Commands,
382    meshes: Query<Entity, (With<Mesh3d>, Without<NotShadowCaster>, Without<Rotate>)>,
383) {
384    for ent in &meshes {
385        commands.entity(ent).insert(NotShadowCaster);
386    }
387}
388
389/// Updates the state of the radio buttons when the user clicks on one.
390fn update_radio_buttons(
391    mut widgets: Query<(
392        Entity,
393        Option<&mut BackgroundColor>,
394        Has<Text>,
395        &WidgetClickSender<Selection>,
396    )>,
397    app_status: Res<AppStatus>,
398    mut writer: TextUiWriter,
399    visible: Query<(&Visibility, &Selection)>,
400    mut visibility_widgets: Query<
401        (
402            Entity,
403            Option<&mut BackgroundColor>,
404            Has<Text>,
405            &WidgetClickSender<Visibility>,
406        ),
407        Without<WidgetClickSender<Selection>>,
408    >,
409) {
410    for (entity, maybe_bg_color, has_text, sender) in &mut widgets {
411        let selected = app_status.selection == **sender;
412        if let Some(mut bg_color) = maybe_bg_color {
413            widgets::update_ui_radio_button(&mut bg_color, selected);
414        }
415        if has_text {
416            widgets::update_ui_radio_button_text(entity, &mut writer, selected);
417        }
418    }
419
420    let visibility = visible
421        .iter()
422        .filter(|(_, selection)| **selection == app_status.selection)
423        .map(|(visibility, _)| *visibility)
424        .next()
425        .unwrap_or_default();
426    for (entity, maybe_bg_color, has_text, sender) in &mut visibility_widgets {
427        if let Some(mut bg_color) = maybe_bg_color {
428            widgets::update_ui_radio_button(&mut bg_color, **sender == visibility);
429        }
430        if has_text {
431            widgets::update_ui_radio_button_text(entity, &mut writer, **sender == visibility);
432        }
433    }
434}
435
436/// Changes the selection when the user clicks a radio button.
437fn handle_selection_change(
438    mut events: MessageReader<WidgetClickEvent<Selection>>,
439    mut app_status: ResMut<AppStatus>,
440) {
441    for event in events.read() {
442        app_status.selection = **event;
443    }
444}
445
446fn toggle_visibility(
447    mut events: MessageReader<WidgetClickEvent<Visibility>>,
448    app_status: Res<AppStatus>,
449    mut visibility: Query<(&mut Visibility, &Selection)>,
450) {
451    if let Some(vis) = events.read().last() {
452        for (mut visibility, selection) in visibility.iter_mut() {
453            if selection == &app_status.selection {
454                *visibility = **vis;
455            }
456        }
457    }
458}
459
460/// Process a drag event that moves the selected object.
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
516
517/// Processes a drag event that scales the selected target.
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
546
547/// Processes a drag event that rotates the selected target along its local Z
548/// axis.
549fn process_roll_input(
550    mut selections: Query<(&mut Transform, &Selection)>,
551    mouse_buttons: Res<ButtonInput<MouseButton>>,
552    mouse_motion: Res<AccumulatedMouseMotion>,
553    app_status: Res<AppStatus>,
554) {
555    // Only process drags when the rolling operation is selected.
556    if !mouse_buttons.pressed(MouseButton::Left) || app_status.drag_mode != DragMode::Roll {
557        return;
558    }
559
560    for (mut transform, selection) in &mut selections {
561        if app_status.selection != *selection {
562            continue;
563        }
564
565        let (yaw, pitch, mut roll) = transform.rotation.to_euler(EulerRot::YXZ);
566        roll += mouse_motion.delta.x * ROLL_SPEED;
567        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
568    }
569}
570
571/// Creates the help string at the top left of the screen.
572fn create_help_string(app_status: &AppStatus) -> String {
573    format!(
574        "Click and drag to {} {}",
575        app_status.drag_mode, app_status.selection
576    )
577}
578
579/// Changes the drag mode when the user hovers over the "Scale" and "Roll"
580/// buttons in the lower right.
581///
582/// If the user is hovering over no such button, this system changes the drag
583/// mode back to its default value of [`DragMode::Move`].
584fn switch_drag_mode(
585    mut commands: Commands,
586    mut interactions: Query<(&Interaction, &DragMode)>,
587    mut windows: Query<Entity, With<Window>>,
588    mouse_buttons: Res<ButtonInput<MouseButton>>,
589    mut app_status: ResMut<AppStatus>,
590) {
591    if mouse_buttons.pressed(MouseButton::Left) {
592        return;
593    }
594
595    for (interaction, drag_mode) in &mut interactions {
596        if *interaction != Interaction::Hovered {
597            continue;
598        }
599
600        app_status.drag_mode = *drag_mode;
601
602        // Set the cursor to provide the user with a nice visual hint.
603        for window in &mut windows {
604            commands
605                .entity(window)
606                .insert(CursorIcon::from(SystemCursorIcon::EwResize));
607        }
608        return;
609    }
610
611    app_status.drag_mode = DragMode::Move;
612
613    for window in &mut windows {
614        commands.entity(window).remove::<CursorIcon>();
615    }
616}
617
618/// Updates the help text in the top left of the screen to reflect the current
619/// selection and drag mode.
620fn update_help_text(mut help_text: Query<&mut Text, With<HelpText>>, app_status: Res<AppStatus>) {
621    for mut text in &mut help_text {
622        text.0 = create_help_string(&app_status);
623    }
624}
625
626/// Updates the visibility of the drag mode buttons so that they aren't visible
627/// if the camera is selected.
628fn update_button_visibility(
629    mut nodes: Query<&mut Visibility, Or<(With<DragMode>, With<WidgetClickSender<Visibility>>)>>,
630    app_status: Res<AppStatus>,
631) {
632    for mut visibility in &mut nodes {
633        *visibility = match app_status.selection {
634            Selection::Camera => Visibility::Hidden,
635            _ => Visibility::Visible,
636        };
637    }
638}
639
640fn update_directional_light(
641    mut commands: Commands,
642    asset_server: Res<AssetServer>,
643    selections: Query<(&Selection, &Visibility)>,
644    mut light: Query<(
645        Entity,
646        &mut DirectionalLight,
647        Option<&DirectionalLightTexture>,
648    )>,
649) {
650    let directional_visible = selections
651        .iter()
652        .filter(|(selection, _)| **selection == Selection::DirectionalLight)
653        .any(|(_, visibility)| visibility != Visibility::Hidden);
654    let any_texture_light_visible = selections
655        .iter()
656        .filter(|(selection, _)| {
657            **selection == Selection::PointLight || **selection == Selection::SpotLight
658        })
659        .any(|(_, visibility)| visibility != Visibility::Hidden);
660
661    let (entity, mut light, maybe_texture) = light
662        .single_mut()
663        .expect("there should be a single directional light");
664
665    if directional_visible {
666        light.illuminance = AMBIENT_DAYLIGHT;
667        if maybe_texture.is_none() {
668            commands.entity(entity).insert(DirectionalLightTexture {
669                image: asset_server.load("lightmaps/caustic_directional_texture.png"),
670                tiled: true,
671            });
672        }
673    } else if any_texture_light_visible {
674        light.illuminance = CLEAR_SUNRISE;
675        if maybe_texture.is_some() {
676            commands.entity(entity).remove::<DirectionalLightTexture>();
677        }
678    } else {
679        light.illuminance = AMBIENT_DAYLIGHT;
680        if maybe_texture.is_some() {
681            commands.entity(entity).remove::<DirectionalLightTexture>();
682        }
683    }
684}
```

examples/asset/asset\_saving\_with\_subassets.rs ([line 316](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#316))

```rust
305fn start_rotate_box_hue(
306    event: On<Pointer<Press>>,
307    boxes: Query<(), With<Box>>,
308    mut commands: Commands,
309) {
310    if event.button != PointerButton::Secondary {
311        return;
312    }
313    if !boxes.contains(event.entity) {
314        return;
315    }
316    commands.entity(event.entity).insert(RotateHue);
317}
```

examples/ui/widgets/standard\_widgets\_observers.rs ([line 454](../../../src/standard_widgets_observers/standard_widgets_observers.rs.html#454))

```rust
445fn update_widget_values(
446    res: Res<DemoWidgetStates>,
447    mut sliders: Query<Entity, With<DemoSlider>>,
448    mut commands: Commands,
449) {
450    if res.is_changed() {
451        for slider_ent in sliders.iter_mut() {
452            commands
453                .entity(slider_ent)
454                .insert(SliderValue(res.slider_value));
455        }
456    }
457}
458
459fn toggle_disabled(
460    input: Res<ButtonInput<KeyCode>>,
461    mut interaction_query: Query<
462        (Entity, Has<InteractionDisabled>),
463        Or<(With<Button>, With<Slider>, With<Checkbox>)>,
464    >,
465    mut commands: Commands,
466) {
467    if input.just_pressed(KeyCode::KeyD) {
468        for (entity, disabled) in &mut interaction_query {
469            if disabled {
470                info!("Widget enabled");
471                commands.entity(entity).remove::<InteractionDisabled>();
472            } else {
473                info!("Widget disabled");
474                commands.entity(entity).insert(InteractionDisabled);
475            }
476        }
477    }
478}
```

examples/3d/irradiance\_volumes.rs ([line 275](../../../src/irradiance_volumes/irradiance_volumes.rs.html#275))

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

Additional examples can be found in:  

*   [examples/ecs/one\_shot\_systems.rs](../../../src/one_shot_systems/one_shot_systems.rs.html#64)
*   [examples/gizmos/transform\_gizmo.rs](../../../src/transform_gizmo/transform_gizmo.rs.html#135)
*   [examples/ecs/extraction.rs](../../../src/extraction/extraction.rs.html#120)
*   [examples/window/screenshot.rs](../../../src/screenshot/screenshot.rs.html#43)
*   [examples/3d/reflection\_probes.rs](../../../src/reflection_probes/reflection_probes.rs.html#198)
*   [examples/3d/lightmaps.rs](../../../src/lightmaps/lightmaps.rs.html#53-58)
*   [examples/ui/widgets/tab\_navigation.rs](../../../src/tab_navigation/tab_navigation.rs.html#56-60)
*   [examples/3d/ssr.rs](../../../src/ssr/ssr.rs.html#274)
*   [examples/ecs/entity\_disabling.rs](../../../src/entity_disabling/entity_disabling.rs.html#50)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../../src/custom_shader_instancing/custom_shader_instancing.rs.html#225-228)
*   [examples/asset/asset\_decompression.rs](../../../src/asset_decompression/asset_decompression.rs.html#131)
*   [examples/animation/morph\_targets.rs](../../../src/morph_targets/morph_targets.rs.html#72)
*   [examples/animation/animated\_mesh\_control.rs](../../../src/animated_mesh_control/animated_mesh_control.rs.html#161)
*   [examples/ecs/error\_handling.rs](../../../src/error_handling/error_handling.rs.html#182)
*   [examples/window/window\_settings.rs](../../../src/window_settings/window_settings.rs.html#178)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#361)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#88-91)
*   [examples/3d/rotate\_environment\_map.rs](../../../src/rotate_environment_map/rotate_environment_map.rs.html#114-118)
*   [examples/ui/widgets/standard\_widgets.rs](../../../src/standard_widgets/standard_widgets.rs.html#118)
*   [examples/3d/anisotropy.rs](../../../src/anisotropy/anisotropy.rs.html#169-177)
*   [examples/showcase/game\_menu.rs](../../../src/game_menu/game_menu.rs.html#356)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#84)
*   [examples/usage/cooldown.rs](../../../src/cooldown/cooldown.rs.html#147)
*   [examples/3d/order\_independent\_transparency.rs](../../../src/order_independent_transparency/order_independent_transparency.rs.html#91)
*   [examples/ecs/delayed\_commands.rs](../../../src/delayed_commands/delayed_commands.rs.html#54)
*   [examples/3d/pccm.rs](../../../src/pccm/pccm.rs.html#188)
*   [examples/3d/depth\_of\_field.rs](../../../src/depth_of_field/depth_of_field.rs.html#83)
*   [examples/3d/shadow\_caster\_receiver.rs](../../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#148)
*   [examples/3d/mirror.rs](../../../src/mirror/mirror.rs.html#641)
*   [examples/audio/soundtrack.rs](../../../src/soundtrack/soundtrack.rs.html#72)
*   [examples/3d/pcss.rs](../../../src/pcss/pcss.rs.html#168)
*   [examples/3d/clearcoat.rs](../../../src/clearcoat/clearcoat.rs.html#123)
*   [examples/showcase/stepping.rs](../../../src/breakout/stepping.rs.html#250)
*   [examples/window/custom\_cursor\_image.rs](../../../src/custom_cursor_image/custom_cursor_image.rs.html#43-63)
*   [examples/ui/relative\_cursor\_position.rs](../../../src/relative_cursor_position/relative_cursor_position.rs.html#47)
*   [tests/window/desktop\_request\_redraw.rs](../../../src/desktop_request_redraw/desktop_request_redraw.rs.html#87)
*   [examples/animation/animated\_mesh.rs](../../../src/animated_mesh/animated_mesh.rs.html#94)
*   [examples/stress\_tests/many\_buttons.rs](../../../src/many_buttons/many_buttons.rs.html#294)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../../src/many_morph_targets/many_morph_targets.rs.html#216-225)
*   [examples/math/bounding\_2d.rs](../../../src/bounding_2d/bounding_2d.rs.html#163)
*   [examples/ecs/relationships.rs](../../../src/relationships/relationships.rs.html#71)
*   [examples/3d/occlusion\_culling.rs](../../../src/occlusion_culling/occlusion_culling.rs.html#276)
*   [examples/animation/eased\_motion.rs](../../../src/eased_motion/eased_motion.rs.html#52)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#96)
*   [examples/3d/visibility\_range.rs](../../../src/visibility_range/visibility_range.rs.html#167-172)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#284)
*   [examples/3d/volumetric\_fog.rs](../../../src/volumetric_fog/volumetric_fog.rs.html#75-79)
*   [examples/animation/animated\_mesh\_events.rs](../../../src/animated_mesh_events/animated_mesh_events.rs.html#189)
*   [examples/async\_tasks/async\_compute.rs](../../../src/async_compute/async_compute.rs.html#113)
*   [examples/animation/animation\_graph.rs](../../../src/animation_graph/animation_graph.rs.html#309-313)
*   [examples/3d/blend\_modes.rs](../../../src/blend_modes/blend_modes.rs.html#306)
*   [examples/3d/light\_probe\_blending.rs](../../../src/light_probe_blending/light_probe_blending.rs.html#534-538)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../../src/transform_hierarchy/transform_hierarchy.rs.html#412)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#386)
*   [examples/3d/deferred\_rendering.rs](../../../src/deferred_rendering/deferred_rendering.rs.html#303)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#183)
*   [examples/3d/meshlet.rs](../../../src/meshlet/meshlet.rs.html#95)
*   [examples/diagnostics/log\_diagnostics.rs](../../../src/log_diagnostics/log_diagnostics.rs.html#198-281)
*   [examples/3d/ssao.rs](../../../src/ssao/ssao.rs.html#162)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#154)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#205-208)
*   [examples/3d/bloom\_3d.rs](../../../src/bloom_3d/bloom_3d.rs.html#214)
*   [examples/2d/bloom\_2d.rs](../../../src/bloom_2d/bloom_2d.rs.html#189)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#210-219)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../../src/multiple_text_inputs/multiple_text_inputs.rs.html#111)
*   [examples/ui/styling/box\_shadow.rs](../../../src/box_shadow/box_shadow.rs.html#158-183)
*   [examples/3d/contact\_shadows.rs](../../../src/contact_shadows/contact_shadows.rs.html#200)
*   [examples/3d/anti\_aliasing.rs](../../../src/anti_aliasing/anti_aliasing.rs.html#141)
*   [examples/animation/animated\_transform.rs](../../../src/animated_transform/animated_transform.rs.html#154-172)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#466)
*   [examples/stress\_tests/many\_cubes.rs](../../../src/many_cubes/many_cubes.rs.html#212)
*   [examples/ui/widgets/feathers\_gallery.rs](../../../src/feathers_gallery/feathers_gallery.rs.html#287)
*   [examples/testbed/full\_ui.rs](../../../src/testbed_full_ui/full_ui.rs.html#43)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1464-1466)

#### pub fn [insert\_if](#method.insert_if)<F>( &mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), condition: F, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity if the predicate returns true.

This is useful for chaining method calls.

##### Example

```rust
#[derive(Component)]
struct StillLoadingStats;
#[derive(Component)]
struct Health(u32);

fn add_health_system(mut commands: Commands, player: Res<PlayerEntity>) {
    commands
        .entity(player.entity)
        .insert_if(Health(10), || !player.is_spectator())
        .remove::<StillLoadingStats>();
}
```

##### [Examples found in repository](#scraped-examples-9)[?](../../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1843](../../../src/testbed_ui/ui.rs.html#1843))

```rust
1804    pub fn setup(mut commands: Commands) {
1805        let radius = percent(33.);
1806        let width = px(10.);
1807
1808        commands.spawn((Camera2d, DespawnOnExit(super::Scene::OuterColor)));
1809        commands
1810            .spawn((
1811                Node {
1812                    display: Display::Grid,
1813                    grid_template_columns: RepeatedGridTrack::px(3, 200.),
1814                    grid_template_rows: RepeatedGridTrack::px(3, 200.),
1815                    margin: UiRect::AUTO,
1816                    ..default()
1817                },
1818                DespawnOnExit(super::Scene::OuterColor),
1819            ))
1820            .with_children(|builder| {
1821                for (border, border_radius, invert) in [
1822                    (UiRect::ZERO, BorderRadius::bottom_right(radius), true),
1823                    (UiRect::top(width), BorderRadius::top(radius), false),
1824                    (UiRect::ZERO, BorderRadius::bottom_left(radius), true),
1825                    (UiRect::left(width), BorderRadius::left(radius), false),
1826                    (UiRect::all(width), BorderRadius::all(radius), true),
1827                    (UiRect::right(width), BorderRadius::right(radius), false),
1828                    (UiRect::ZERO, BorderRadius::top_right(radius), true),
1829                    (UiRect::bottom(width), BorderRadius::bottom(radius), false),
1830                    (UiRect::ZERO, BorderRadius::top_left(radius), true),
1831                ] {
1832                    builder
1833                        .spawn((
1834                            Node {
1835                                width: px(200.),
1836                                height: px(200.),
1837                                border_radius,
1838                                border,
1839                                ..default()
1840                            },
1841                            BorderColor::all(bevy::color::palettes::css::RED),
1842                        ))
1843                        .insert_if(BackgroundColor(Color::WHITE), || !invert)
1844                        .insert_if(OuterColor(Color::WHITE), || invert);
1845                }
1846            });
1847    }
```

Hide additional examples

examples/3d/motion\_blur.rs ([line 170](../../../src/motion_blur/motion_blur.rs.html#170))

```rust
105fn spawn_cars(
106    asset_server: &AssetServer,
107    meshes: &mut Assets<Mesh>,
108    materials: &mut Assets<StandardMaterial>,
109    commands: &mut Commands,
110) {
111    const N_CARS: usize = 20;
112    let box_mesh = meshes.add(Cuboid::new(0.3, 0.15, 0.55));
113    let cylinder = meshes.add(Cylinder::default());
114    let logo = asset_server.load("branding/icon.png");
115    let wheel_matl = materials.add(StandardMaterial {
116        base_color: Color::WHITE,
117        base_color_texture: Some(logo.clone()),
118        ..default()
119    });
120
121    let mut matl = |color| {
122        materials.add(StandardMaterial {
123            base_color: color,
124            ..default()
125        })
126    };
127
128    let colors = [
129        matl(Color::linear_rgb(1.0, 0.0, 0.0)),
130        matl(Color::linear_rgb(1.0, 1.0, 0.0)),
131        matl(Color::BLACK),
132        matl(Color::linear_rgb(0.0, 0.0, 1.0)),
133        matl(Color::linear_rgb(0.0, 1.0, 0.0)),
134        matl(Color::linear_rgb(1.0, 0.0, 1.0)),
135        matl(Color::linear_rgb(0.5, 0.5, 0.0)),
136        matl(Color::linear_rgb(1.0, 0.5, 0.0)),
137    ];
138
139    let make_wheel = |x: f32, z: f32| {
140        (
141            Mesh3d(cylinder.clone()),
142            MeshMaterial3d(wheel_matl.clone()),
143            Transform::from_xyz(0.14 * x, -0.045, 0.15 * z)
144                .with_scale(Vec3::new(0.15, 0.04, 0.15))
145                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
146            Rotates,
147        )
148    };
149
150    for i in 0..N_CARS {
151        let color = colors[i % colors.len()].clone();
152        commands
153            .spawn((
154                Mesh3d(box_mesh.clone()),
155                MeshMaterial3d(color.clone()),
156                Transform::from_scale(Vec3::splat(0.5)),
157                Moves(i as f32 * 2.0),
158                children![
159                    (
160                        Mesh3d(box_mesh.clone()),
161                        MeshMaterial3d(color),
162                        Transform::from_xyz(0.0, 0.08, 0.03).with_scale(Vec3::new(1.0, 1.0, 0.5)),
163                    ),
164                    make_wheel(1.0, 1.0),
165                    make_wheel(1.0, -1.0),
166                    make_wheel(-1.0, 1.0),
167                    make_wheel(-1.0, -1.0)
168                ],
169            ))
170            .insert_if(CameraTracked, || i == 0);
171    }
172}
```

examples/3d/ssao.rs ([lines 112-118](../../../src/ssao/ssao.rs.html#112-118))

```rust
90fn update(
91    camera: Single<
92        (
93            Entity,
94            Option<&ScreenSpaceAmbientOcclusion>,
95            Option<&TemporalJitter>,
96        ),
97        With<Camera>,
98    >,
99    mut text: Single<&mut Text>,
100    mut sphere: Single<&mut Transform, With<SphereMarker>>,
101    mut commands: Commands,
102    keycode: Res<ButtonInput<KeyCode>>,
103    time: Res<Time>,
104) {
105    sphere.translation.y = ops::sin(time.elapsed_secs() / 1.7) * 0.7;
106
107    let (camera_entity, ssao, temporal_jitter) = *camera;
108    let current_ssao = ssao.cloned().unwrap_or_default();
109
110    let mut commands = commands.entity(camera_entity);
111    commands
112        .insert_if(
113            ScreenSpaceAmbientOcclusion {
114                quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Low,
115                ..current_ssao
116            },
117            || keycode.just_pressed(KeyCode::Digit2),
118        )
119        .insert_if(
120            ScreenSpaceAmbientOcclusion {
121                quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Medium,
122                ..current_ssao
123            },
124            || keycode.just_pressed(KeyCode::Digit3),
125        )
126        .insert_if(
127            ScreenSpaceAmbientOcclusion {
128                quality_level: ScreenSpaceAmbientOcclusionQualityLevel::High,
129                ..current_ssao
130            },
131            || keycode.just_pressed(KeyCode::Digit4),
132        )
133        .insert_if(
134            ScreenSpaceAmbientOcclusion {
135                quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
136                ..current_ssao
137            },
138            || keycode.just_pressed(KeyCode::Digit5),
139        )
140        .insert_if(
141            ScreenSpaceAmbientOcclusion {
142                constant_object_thickness: (current_ssao.constant_object_thickness * 2.0).min(4.0),
143                ..current_ssao
144            },
145            || keycode.just_pressed(KeyCode::ArrowUp),
146        )
147        .insert_if(
148            ScreenSpaceAmbientOcclusion {
149                constant_object_thickness: (current_ssao.constant_object_thickness * 0.5)
150                    .max(0.0625),
151                ..current_ssao
152            },
153            || keycode.just_pressed(KeyCode::ArrowDown),
154        );
155    if keycode.just_pressed(KeyCode::Digit1) {
156        commands.remove::<ScreenSpaceAmbientOcclusion>();
157    }
158    if keycode.just_pressed(KeyCode::Space) {
159        if temporal_jitter.is_some() {
160            commands.remove::<TemporalJitter>();
161        } else {
162            commands.insert(TemporalJitter::default());
163        }
164    }
165
166    text.clear();
167
168    let (o, l, m, h, u) = match ssao.map(|s| s.quality_level) {
169        None => ("*", "", "", "", ""),
170        Some(ScreenSpaceAmbientOcclusionQualityLevel::Low) => ("", "*", "", "", ""),
171        Some(ScreenSpaceAmbientOcclusionQualityLevel::Medium) => ("", "", "*", "", ""),
172        Some(ScreenSpaceAmbientOcclusionQualityLevel::High) => ("", "", "", "*", ""),
173        Some(ScreenSpaceAmbientOcclusionQualityLevel::Ultra) => ("", "", "", "", "*"),
174        _ => unreachable!(),
175    };
176
177    if let Some(thickness) = ssao.map(|s| s.constant_object_thickness) {
178        text.push_str(&format!(
179            "Constant object thickness: {thickness} (Up/Down)\n\n"
180        ));
181    }
182
183    text.push_str("SSAO Quality:\n");
184    text.push_str(&format!("(1) {o}Off{o}\n"));
185    text.push_str(&format!("(2) {l}Low{l}\n"));
186    text.push_str(&format!("(3) {m}Medium{m}\n"));
187    text.push_str(&format!("(4) {h}High{h}\n"));
188    text.push_str(&format!("(5) {u}Ultra{u}\n\n"));
189
190    text.push_str("Temporal Antialiasing:\n");
191    text.push_str(match temporal_jitter {
192        Some(_) => "(Space) Enabled",
193        None => "(Space) Disabled",
194    });
195}
```

examples/3d/solari.rs ([line 261](../../../src/solari/solari.rs.html#261))

```rust
200fn setup_many_lights(
201    mut commands: Commands,
202    asset_server: Res<AssetServer>,
203    mut meshes: ResMut<Assets<Mesh>>,
204    mut materials: ResMut<Assets<StandardMaterial>>,
205    args: Res<Args>,
206    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))] dlss_rr_supported: Option<
207        Res<DlssRayReconstructionSupported>,
208    >,
209) {
210    let mut rng = ChaCha8Rng::seed_from_u64(42);
211
212    let mut plane_mesh = Plane3d::default()
213        .mesh()
214        .size(400.0, 400.0)
215        .build()
216        .with_generated_tangents()
217        .unwrap();
218    match plane_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap() {
219        VertexAttributeValues::Float32x2(items) => {
220            items.iter_mut().flatten().for_each(|x| *x *= 3.0);
221        }
222        _ => unreachable!(),
223    }
224    let plane_mesh = meshes.add(plane_mesh);
225    let cube_mesh = meshes.add(
226        Cuboid::default()
227            .mesh()
228            .build()
229            .with_generated_tangents()
230            .unwrap(),
231    );
232    let sphere_mesh = meshes.add(
233        Sphere::new(1.0)
234            .mesh()
235            .build()
236            .with_generated_tangents()
237            .unwrap(),
238    );
239
240    commands
241        .spawn((
242            RaytracingMesh3d(plane_mesh.clone()),
243            MeshMaterial3d(
244                materials.add(StandardMaterial {
245                    base_color_texture: Some(
246                        asset_server
247                            .load_builder()
248                            .with_settings::<ImageLoaderSettings>(|settings| {
249                                settings
250                                    .sampler
251                                    .get_or_init_descriptor()
252                                    .set_address_mode(ImageAddressMode::Repeat);
253                            })
254                            .load("textures/uv_checker_bw.png"),
255                    ),
256                    perceptual_roughness: 0.0,
257                    ..default()
258                }),
259            ),
260        ))
261        .insert_if(Mesh3d(plane_mesh), || args.pathtracer != Some(true));
262
263    for _ in 0..8000 {
264        commands
265            .spawn((
266                RaytracingMesh3d(cube_mesh.clone()),
267                MeshMaterial3d(materials.add(StandardMaterial {
268                    base_color: Color::srgb(rng.random(), rng.random(), rng.random()),
269                    perceptual_roughness: rng.random(),
270                    ..default()
271                })),
272                Transform::default()
273                    .with_scale(Vec3 {
274                        x: rng.random_range(0.2..=2.0),
275                        y: rng.random_range(0.2..=2.0),
276                        z: rng.random_range(0.2..=2.0),
277                    })
278                    .with_translation(Vec3::new(
279                        rng.random_range(-180.0..=180.0),
280                        0.2,
281                        rng.random_range(-180.0..=180.0),
282                    )),
283            ))
284            .insert_if(Mesh3d(cube_mesh.clone()), || args.pathtracer != Some(true));
285    }
286
287    for x in -10..=10 {
288        for y in -10..=10 {
289            commands
290                .spawn((
291                    RaytracingMesh3d(sphere_mesh.clone()),
292                    MeshMaterial3d(
293                        materials.add(StandardMaterial {
294                            emissive: Color::linear_rgb(
295                                rng.random::<f32>() * 60000.0,
296                                rng.random::<f32>() * 60000.0,
297                                rng.random::<f32>() * 60000.0,
298                            )
299                            .into(),
300                            ..default()
301                        }),
302                    ),
303                    Transform::default().with_translation(Vec3::new(
304                        (x * 20) as f32,
305                        7.0,
306                        (y * 20) as f32,
307                    )),
308                ))
309                .insert_if(Mesh3d(sphere_mesh.clone()), || {
310                    args.pathtracer != Some(true)
311                });
312        }
313    }
314
315    let mut camera = commands.spawn((
316        Camera3d::default(),
317        Camera {
318            clear_color: ClearColorConfig::Custom(Color::BLACK),
319            ..default()
320        },
321        FreeCamera {
322            walk_speed: 3.0,
323            run_speed: 10.0,
324            ..Default::default()
325        },
326        Transform::from_translation(Vec3::new(6.11329, 166.74896, 451.8226)).with_rotation(
327            Quat::from_xyzw(-0.183938, 0.009093744, 0.0017017953, 0.9828943),
328        ),
329        // Msaa::Off and CameraMainTextureUsages with STORAGE_BINDING are required for Solari
330        CameraMainTextureUsages::default().with(TextureUsages::STORAGE_BINDING),
331        Msaa::Off,
332        Bloom {
333            intensity: 0.1,
334            ..Bloom::NATURAL
335        },
336    ));
337
338    if args.pathtracer == Some(true) {
339        camera.insert(Pathtracer::default());
340    } else {
341        camera.insert(SolariLighting::default());
342    }
343
344    // Using DLSS Ray Reconstruction for denoising (and cheaper rendering via upscaling) is _highly_ recommended when using Solari
345    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))]
346    if dlss_rr_supported.is_some() {
347        camera.insert(Dlss::<DlssRayReconstructionFeature> {
348            perf_quality_mode: Default::default(),
349            reset: Default::default(),
350            _phantom_data: Default::default(),
351        });
352    }
353
354    commands.spawn((
355        Node {
356            position_type: PositionType::Absolute,
357            right: px(0.0),
358            padding: px(4.0).all(),
359            border_radius: BorderRadius::bottom_left(px(4.0)),
360            ..default()
361        },
362        BackgroundColor(Color::srgba(0.10, 0.10, 0.10, 0.8)),
363        children![(
364            PerformanceText,
365            Text::default(),
366            TextFont {
367                font_size: FontSize::Px(8.0),
368                ..default()
369            },
370        )],
371    ));
372}
```

examples/stress\_tests/many\_cubes.rs ([line 204](../../../src/many_cubes/many_cubes.rs.html#204))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1483)

#### pub fn [insert\_if\_new](#method.insert_if_new)(&mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity without overwriting.

This is the same as [`EntityCommands::insert`](../../prelude/struct.EntityCommands.html#method.insert "method bevy::prelude::EntityCommands::insert"), but in case of duplicate components will leave the old values instead of replacing them with new ones.

See also [`entry`](../../prelude/struct.EntityCommands.html#method.entry "method bevy::prelude::EntityCommands::entry"), which lets you modify a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if it’s present, as well as initialize it with a default value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1493-1495)

#### pub fn [insert\_if\_new\_and](#method.insert_if_new_and)<F>( &mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), condition: F, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity without overwriting if the predicate returns true.

This is the same as [`EntityCommands::insert_if`](../../prelude/struct.EntityCommands.html#method.insert_if "method bevy::prelude::EntityCommands::insert_if"), but in case of duplicate components will leave the old values instead of replacing them with new ones.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1507)

#### pub fn [insert\_if\_neq](#method.insert_if_neq)<T>(&mut self, component: T) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Adds a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to the entity if the component is different or missing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1537-1541)

#### pub unsafe fn [insert\_by\_id](#method.insert_by_id)<T>( &mut self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), value: T, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Adds a dynamic [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to the entity.

This will overwrite any previous value(s) of the same component type.

You should prefer to use the typed API [`EntityCommands::insert`](../../prelude/struct.EntityCommands.html#method.insert "method bevy::prelude::EntityCommands::insert") where possible.

##### Safety

*   [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") must be from the same world as `self`.
*   `T` must have the same layout as the one passed during `component_id` creation.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1566-1570)

#### pub unsafe fn [try\_insert\_by\_id](#method.try_insert_by_id)<T>( &mut self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), value: T, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Adds a dynamic [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to the entity.

This will overwrite any previous value(s) of the same component type.

You should prefer to use the typed API [`EntityCommands::try_insert`](../../prelude/struct.EntityCommands.html#method.try_insert "method bevy::prelude::EntityCommands::try_insert") where possible.

##### Note

If the entity does not exist when this command is executed, the resulting error will be ignored.

##### Safety

*   [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") must be from the same world as `self`.
*   `T` must have the same layout as the one passed during `component_id` creation.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1626)

#### pub fn [try\_insert](#method.try_insert)(&mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity.

This will overwrite any previous value(s) of the same component type.

##### Note

If the entity does not exist when this command is executed, the resulting error will be ignored.

##### Example

```rust
#[derive(Component)]
struct Health(u32);
#[derive(Component)]
struct Strength(u32);
#[derive(Component)]
struct Defense(u32);

#[derive(Bundle)]
struct CombatBundle {
    health: Health,
    strength: Strength,
}

fn add_combat_stats_system(mut commands: Commands, player: Res<PlayerEntity>) {
    commands.entity(player.entity)
        // You can insert individual components:
        .try_insert(Defense(10))
        // You can also insert tuples of components:
        .try_insert(CombatBundle {
            health: Health(100),
            strength: Strength(40),
        });

    // Suppose this occurs in a parallel adjacent system or process.
    commands.entity(player.entity).despawn();

    // This will not panic nor will it add the component.
    commands.entity(player.entity).try_insert(Defense(5));
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1639-1641)

#### pub fn [try\_insert\_if](#method.try_insert_if)<F>( &mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), condition: F, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity if the predicate returns true.

This is useful for chaining method calls.

##### Note

If the entity does not exist when this command is executed, the resulting error will be ignored.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1661-1663)

#### pub fn [try\_insert\_if\_new\_and](#method.try_insert_if_new_and)<F>( &mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), condition: F, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity without overwriting if the predicate returns true.

This is the same as [`EntityCommands::try_insert_if`](../../prelude/struct.EntityCommands.html#method.try_insert_if "method bevy::prelude::EntityCommands::try_insert_if"), but in case of duplicate components will leave the old values instead of replacing them with new ones.

##### Note

If the entity does not exist when this command is executed, the resulting error will be ignored.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1682)

#### pub fn [try\_insert\_if\_new](#method.try_insert_if_new)( &mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity without overwriting.

This is the same as [`EntityCommands::try_insert`](../../prelude/struct.EntityCommands.html#method.try_insert "method bevy::prelude::EntityCommands::try_insert"), but in case of duplicate components will leave the old values instead of replacing them with new ones.

##### Note

If the entity does not exist when this command is executed, the resulting error will be ignored.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1726)

#### pub fn [remove](#method.remove)<B>(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components from the entity.

This will remove all components that intersect with the provided bundle; the entity does not need to have all the components in the bundle.

This will emit a warning if the entity does not exist.

##### Example

```rust
#[derive(Component)]
struct Health(u32);
#[derive(Component)]
struct Strength(u32);
#[derive(Component)]
struct Defense(u32);

#[derive(Bundle)]
struct CombatBundle {
    health: Health,
    strength: Strength,
}

fn remove_combat_stats_system(mut commands: Commands, player: Res<PlayerEntity>) {
    commands
        .entity(player.entity)
        // You can remove individual components:
        .remove::<Defense>()
        // You can also remove pre-defined bundles of components:
        .remove::<CombatBundle>()
        // You can also remove tuples of components and bundles.
        // This is equivalent to the calls above:
        .remove::<(Defense, CombatBundle)>();
}
```

##### [Examples found in repository](#scraped-examples-10)[?](../../../scrape-examples-help.html)

examples/remote/server.rs ([line 86](../../../src/server/server.rs.html#86))

```rust
85fn remove(mut commands: Commands, cube_entity: Single<Entity, With<Cube>>) {
86    commands.entity(*cube_entity).remove::<Cube>();
87}
```

Hide additional examples

examples/ecs/one\_shot\_systems.rs ([line 78](../../../src/one_shot_systems/one_shot_systems.rs.html#78))

```rust
75fn evaluate_callbacks(query: Query<(Entity, &Callback), With<Triggered>>, mut commands: Commands) {
76    for (entity, callback) in query.iter() {
77        commands.run_system(callback.0);
78        commands.entity(entity).remove::<Triggered>();
79    }
80}
```

examples/ecs/removal\_detection.rs ([line 47](../../../src/removal_detection/removal_detection.rs.html#47))

```rust
38fn remove_component(
39    time: Res<Time>,
40    mut commands: Commands,
41    query: Query<Entity, With<MyComponent>>,
42) {
43    // After two seconds have passed the `Component` is removed.
44    if time.elapsed_secs() > 2.0
45        && let Some(entity) = query.iter().next()
46    {
47        commands.entity(entity).remove::<MyComponent>();
48    }
49}
```

examples/asset/asset\_saving\_with\_subassets.rs ([line 331](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#331))

```rust
320fn end_rotate_box_hue_on_release(
321    event: On<Pointer<Release>>,
322    boxes: Query<(), (With<Box>, With<RotateHue>)>,
323    mut commands: Commands,
324) {
325    if event.button != PointerButton::Secondary {
326        return;
327    }
328    if !boxes.contains(event.entity) {
329        return;
330    }
331    commands.entity(event.entity).remove::<RotateHue>();
332}
333
334/// Stops rotating the box hue if the cursor moves off the entity.
335fn end_rotate_box_hue_on_out(
336    event: On<Pointer<Out>>,
337    boxes: Query<(), (With<Box>, With<RotateHue>)>,
338    mut commands: Commands,
339) {
340    if !boxes.contains(event.entity) {
341        return;
342    }
343    commands.entity(event.entity).remove::<RotateHue>();
344}
```

examples/ecs/component\_hooks.rs ([line 143](../../../src/component_hooks/component_hooks.rs.html#143))

```rust
136fn trigger_hooks(
137    mut commands: Commands,
138    keys: Res<ButtonInput<KeyCode>>,
139    index: Res<MyComponentIndex>,
140) {
141    for (key, entity) in index.iter() {
142        if !keys.pressed(*key) {
143            commands.entity(*entity).remove::<MyComponent>();
144        }
145    }
146    for key in keys.get_just_pressed() {
147        commands.spawn(MyComponent(*key));
148    }
149}
```

examples/gizmos/transform\_gizmo.rs ([line 132](../../../src/transform_gizmo/transform_gizmo.rs.html#132))

```rust
122fn on_click_select(
123    click: On<Pointer<Click>>,
124    mut commands: Commands,
125    existing: Query<Entity, With<TransformGizmoFocus>>,
126) {
127    if click.button != PointerButton::Primary {
128        return;
129    }
130    // Remove focus from all entities
131    for e in &existing {
132        commands.entity(e).remove::<TransformGizmoFocus>();
133    }
134    // Add focus to clicked entity
135    commands.entity(click.entity).insert(TransformGizmoFocus);
136}
```

Additional examples can be found in:  

*   [examples/window/screenshot.rs](../../../src/screenshot/screenshot.rs.html#38)
*   [examples/audio/soundtrack.rs](../../../src/soundtrack/soundtrack.rs.html#112)
*   [examples/3d/depth\_of\_field.rs](../../../src/depth_of_field/depth_of_field.rs.html#170)
*   [examples/ecs/entity\_disabling.rs](../../../src/entity_disabling/entity_disabling.rs.html#97)
*   [examples/ui/widgets/tab\_navigation.rs](../../../src/tab_navigation/tab_navigation.rs.html#62)
*   [examples/ecs/relationships.rs](../../../src/relationships/relationships.rs.html#206)
*   [examples/async\_tasks/async\_compute.rs](../../../src/async_compute/async_compute.rs.html#138)
*   [examples/asset/asset\_decompression.rs](../../../src/asset_decompression/asset_decompression.rs.html#130)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../../src/standard_widgets_observers/standard_widgets_observers.rs.html#471)
*   [examples/usage/cooldown.rs](../../../src/cooldown/cooldown.rs.html#171)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#290)
*   [examples/3d/visibility\_range.rs](../../../src/visibility_range/visibility_range.rs.html#339)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#87)
*   [examples/ui/widgets/standard\_widgets.rs](../../../src/standard_widgets/standard_widgets.rs.html#128)
*   [examples/3d/clearcoat.rs](../../../src/clearcoat/clearcoat.rs.html#284)
*   [examples/3d/clustered\_decals.rs](../../../src/clustered_decals/clustered_decals.rs.html#521)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#614)
*   [examples/3d/occlusion\_culling.rs](../../../src/occlusion_culling/occlusion_culling.rs.html#663)
*   [examples/showcase/game\_menu.rs](../../../src/game_menu/game_menu.rs.html#355)
*   [examples/3d/order\_independent\_transparency.rs](../../../src/order_independent_transparency/order_independent_transparency.rs.html#84)
*   [examples/3d/irradiance\_volumes.rs](../../../src/irradiance_volumes/irradiance_volumes.rs.html#431)
*   [examples/3d/shadow\_caster\_receiver.rs](../../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#145)
*   [examples/3d/pcss.rs](../../../src/pcss/pcss.rs.html#298)
*   [tests/window/desktop\_request\_redraw.rs](../../../src/desktop_request_redraw/desktop_request_redraw.rs.html#90)
*   [examples/3d/volumetric\_fog.rs](../../../src/volumetric_fog/volumetric_fog.rs.html#253)
*   [examples/3d/reflection\_probes.rs](../../../src/reflection_probes/reflection_probes.rs.html#248)
*   [examples/3d/blend\_modes.rs](../../../src/blend_modes/blend_modes.rs.html#304)
*   [examples/3d/light\_probe\_blending.rs](../../../src/light_probe_blending/light_probe_blending.rs.html#533)
*   [examples/3d/anisotropy.rs](../../../src/anisotropy/anisotropy.rs.html#256)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#395)
*   [examples/3d/deferred\_rendering.rs](../../../src/deferred_rendering/deferred_rendering.rs.html#302)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#420)
*   [examples/3d/mixed\_lighting.rs](../../../src/mixed_lighting/mixed_lighting.rs.html#273)
*   [examples/3d/ssao.rs](../../../src/ssao/ssao.rs.html#156)
*   [examples/3d/contact\_shadows.rs](../../../src/contact_shadows/contact_shadows.rs.html#447)
*   [examples/3d/bloom\_3d.rs](../../../src/bloom_3d/bloom_3d.rs.html#140)
*   [examples/2d/bloom\_2d.rs](../../../src/bloom_2d/bloom_2d.rs.html#115)
*   [examples/3d/anti\_aliasing.rs](../../../src/anti_aliasing/anti_aliasing.rs.html#104)
*   [examples/3d/ssr.rs](../../../src/ssr/ssr.rs.html#747)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#464)
*   [examples/ui/widgets/feathers\_gallery.rs](../../../src/feathers_gallery/feathers_gallery.rs.html#289)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1762)

#### pub fn [remove\_if](#method.remove_if)<B>( &mut self, condition: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components from the entity if the predicate returns true.

This is useful for chaining method calls.

##### Example

```rust
#[derive(Component)]
struct Health(u32);
#[derive(Component)]
struct Strength(u32);
#[derive(Component)]
struct Defense(u32);

#[derive(Bundle)]
struct CombatBundle {
    health: Health,
    strength: Strength,
}

fn remove_combat_stats_system(mut commands: Commands, player: Res<PlayerEntity>) {
    commands
        .entity(player.entity)
        .remove_if::<(Defense, CombatBundle)>(|| !player.is_spectator());
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1779)

#### pub fn [try\_remove\_if](#method.try_remove_if)<B>( &mut self, condition: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components from the entity if the predicate returns true.

This is useful for chaining method calls.

##### Note

If the entity does not exist when this command is executed, the resulting error will be ignored.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1827)

#### pub fn [try\_remove](#method.try_remove)<B>(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components from the entity.

This will remove all components that intersect with the provided bundle; the entity does not need to have all the components in the bundle.

Unlike [`Self::remove`](../../prelude/struct.EntityCommands.html#method.remove "method bevy::prelude::EntityCommands::remove"), this will not emit a warning if the entity does not exist.

##### Example

```rust
#[derive(Component)]
struct Health(u32);
#[derive(Component)]
struct Strength(u32);
#[derive(Component)]
struct Defense(u32);

#[derive(Bundle)]
struct CombatBundle {
    health: Health,
    strength: Strength,
}

fn remove_combat_stats_system(mut commands: Commands, player: Res<PlayerEntity>) {
    commands
        .entity(player.entity)
        // You can remove individual components:
        .try_remove::<Defense>()
        // You can also remove pre-defined bundles of components:
        .try_remove::<CombatBundle>()
        // You can also remove tuples of components and bundles.
        // This is equivalent to the calls above:
        .try_remove::<(Defense, CombatBundle)>();
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1859)

#### pub fn [remove\_with\_requires](#method.remove_with_requires)<B>(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components from the entity, and also removes any components required by the components in the bundle.

This will remove all components that intersect with the provided bundle; the entity does not need to have all the components in the bundle.

##### Example

```rust
#[derive(Component)]
#[require(B)]
struct A;
#[derive(Component, Default)]
struct B;

fn remove_with_requires_system(mut commands: Commands, player: Res<PlayerEntity>) {
    commands
        .entity(player.entity)
        // Removes both A and B from the entity, because B is required by A.
        .remove_with_requires::<A>();
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1869)

#### pub fn [remove\_by\_id](#method.remove_by_id)( &mut self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Removes a dynamic [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") from the entity if it exists.

##### Panics

Panics if the provided [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") does not exist in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1875)

#### pub fn [clear](#method.clear)(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Removes all components associated with the entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1906)

#### pub fn [despawn](#method.despawn)(&mut self)

Despawns the entity.

This will emit a warning if the entity does not exist.

##### Note

This will also despawn the entities in any [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") that is configured to despawn descendants.

For example, this will recursively despawn [`Children`](../../prelude/struct.Children.html "struct bevy::prelude::Children").

##### Example

```rust
fn remove_character_system(
    mut commands: Commands,
    character_to_remove: Res<CharacterToRemove>
) {
    commands.entity(character_to_remove.entity).despawn();
}
```

##### [Examples found in repository](#scraped-examples-11)[?](../../../scrape-examples-help.html)

examples/state/custom\_transitions.rs ([line 166](../../../src/custom_transitions/custom_transitions.rs.html#166))

```rust
165fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
166    commands.entity(menu_data.button_entity).despawn();
167}
168
169const SPEED: f32 = 100.0;
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
195
196fn change_color(time: Res<Time>, mut query: Query<&mut Sprite>) {
197    for mut sprite in &mut query {
198        let new_color = LinearRgba {
199            blue: ops::sin(time.elapsed_secs() * 0.5) + 2.0,
200            ..LinearRgba::from(sprite.color)
201        };
202
203        sprite.color = new_color.into();
204    }
205}
206
207// We can restart the game by pressing "R".
208// This will trigger an [`AppState::InGame`] -> [`AppState::InGame`]
209// transition, which will run our custom schedules.
210fn trigger_game_restart(
211    input: Res<ButtonInput<KeyCode>>,
212    mut next_state: ResMut<NextState<AppState>>,
213) {
214    if input.just_pressed(KeyCode::KeyR) {
215        // Although we are already in this state setting it again will generate an identity transition.
216        // While default schedules ignore those kinds of transitions, our custom schedules will react to them.
217        next_state.set(AppState::InGame);
218    }
219}
220
221fn setup(mut commands: Commands) {
222    commands.spawn(Camera2d);
223}
224
225fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
226    commands.spawn(Sprite::from_image(asset_server.load("branding/icon.png")));
227    info!("Setup game");
228}
229
230fn teardown_game(mut commands: Commands, player: Single<Entity, With<Sprite>>) {
231    commands.entity(*player).despawn();
232    info!("Teardown game");
233}
```

Hide additional examples

examples/state/states.rs ([line 118](../../../src/states/states.rs.html#118))

```rust
117fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
118    commands.entity(menu_data.button_entity).despawn();
119}
```

examples/state/sub\_states.rs ([line 87](../../../src/sub_states/sub_states.rs.html#87))

```rust
86fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
87    commands.entity(menu_data.button_entity).despawn();
88}
```

examples/state/computed\_states.rs ([line 406](../../../src/computed_states/computed_states.rs.html#406))

```rust
405    pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
406        commands.entity(menu_data.root_entity).despawn();
407    }
```

examples/stress\_tests/many\_buttons.rs ([line 322](../../../src/many_buttons/many_buttons.rs.html#322))

```rust
321fn despawn_ui(mut commands: Commands, root_node: Single<Entity, (With<Node>, Without<ChildOf>)>) {
322    commands.entity(*root_node).despawn();
323}
```

examples/ecs/generic\_system.rs ([line 87](../../../src/generic_system/generic_system.rs.html#87))

```rust
85fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
86    for e in &query {
87        commands.entity(e).despawn();
88    }
89}
```

Additional examples can be found in:  

*   [examples/usage/context\_menu.rs](../../../src/context_menu/context_menu.rs.html#77)
*   [examples/showcase/loading\_screen.rs](../../../src/loading_screen/loading_screen.rs.html#128)
*   [examples/ecs/contiguous\_query.rs](../../../src/contiguous_query/contiguous_query.rs.html#43)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../../src/external_source_external_thread/external_source_external_thread.rs.html#71)
*   [examples/asset/multi\_asset\_sync.rs](../../../src/multi_asset_sync/multi_asset_sync.rs.html#274)
*   [examples/audio/soundtrack.rs](../../../src/soundtrack/soundtrack.rs.html#129)
*   [examples/ecs/observers.rs](../../../src/observers/observers.rs.html#169)
*   [examples/ecs/observer\_propagation.rs](../../../src/observer_propagation/observer_propagation.rs.html#117)
*   [examples/3d/pbr.rs](../../../src/pbr/pbr.rs.html#142)
*   [examples/animation/animated\_mesh\_events.rs](../../../src/animated_mesh_events/animated_mesh_events.rs.html#201)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#290)
*   [examples/picking/dragdrop\_picking.rs](../../../src/dragdrop_picking/dragdrop_picking.rs.html#147)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#113)
*   [examples/3d/tonemapping.rs](../../../src/tonemapping/tonemapping.rs.html#221)
*   [examples/ui/widgets/standard\_widgets.rs](../../../src/standard_widgets/standard_widgets.rs.html#805)
*   [examples/3d/order\_independent\_transparency.rs](../../../src/order_independent_transparency/order_independent_transparency.rs.html#109)
*   [examples/ecs/fallible\_params.rs](../../../src/fallible_params/fallible_params.rs.html#118)
*   [examples/ecs/hierarchy.rs](../../../src/hierarchy/hierarchy.rs.html#84)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#464)
*   [examples/2d/dynamic\_mip\_generation.rs](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#568)
*   [examples/3d/clustered\_decal\_maps.rs](../../../src/clustered_decal_maps/clustered_decal_maps.rs.html#373)
*   [examples/showcase/alien\_cake\_addict.rs](../../../src/alien_cake_addict/alien_cake_addict.rs.html#262)
*   [examples/showcase/breakout.rs](../../../src/breakout/breakout.rs.html#357)
*   [examples/window/monitor\_info.rs](../../../src/monitor_info/monitor_info.rs.html#75)
*   [examples/3d/reflection\_probes.rs](../../../src/reflection_probes/reflection_probes.rs.html#229)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../../src/many_morph_targets/many_morph_targets.rs.html#327)
*   [examples/math/random\_sampling.rs](../../../src/random_sampling/random_sampling.rs.html#150)
*   [examples/3d/anisotropy.rs](../../../src/anisotropy/anisotropy.rs.html#232)
*   [examples/ecs/component\_hooks.rs](../../../src/component_hooks/component_hooks.rs.html#131)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../../src/scroll/scroll.rs.html#178)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#190)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1921)

#### pub fn [try\_despawn](#method.try_despawn)(&mut self)

Despawns the entity.

Unlike [`Self::despawn`](../../prelude/struct.EntityCommands.html#method.despawn "method bevy::prelude::EntityCommands::despawn"), this will not emit a warning if the entity does not exist.

##### Note

This will also despawn the entities in any [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") that is configured to despawn descendants.

For example, this will recursively despawn [`Children`](../../prelude/struct.Children.html "struct bevy::prelude::Children").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1956)

#### pub fn [queue](#method.queue)(&mut self, command: impl [EntityCommand](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Pushes an [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") to the queue, which will get executed for the current [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

The [fallback error handler](../error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler") will be used to handle error cases. Every [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") checks whether the entity exists at the time of execution and returns an error if it does not.

To use a custom error handler, see [`EntityCommands::queue_handled`](../../prelude/struct.EntityCommands.html#method.queue_handled "method bevy::prelude::EntityCommands::queue_handled").

The command can be:

*   A custom struct that implements [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand").
*   A closure or function that matches the following signature:
    *   [`(EntityWorldMut)`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")
    *   [`(EntityWorldMut)`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") `->` [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")
*   A built-in command from the [`entity_command`](../system/entity_command/index.html "mod bevy::ecs::system::entity_command") module.

##### Example

```rust
commands
    .spawn_empty()
    // Closures with this signature implement `EntityCommand`.
    .queue(|entity: EntityWorldMut| {
        println!("Executed an EntityCommand for {}", entity.id());
    });
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1998-2002)

#### pub fn [queue\_handled](#method.queue_handled)( &mut self, command: impl [EntityCommand](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand"), error\_handler: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError"), [ErrorContext](../error/enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext")), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Pushes an [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") to the queue, which will get executed for the current [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

The given `error_handler` will be used to handle error cases. Every [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") checks whether the entity exists at the time of execution and returns an error if it does not.

To implicitly use the fallback error handler, see [`EntityCommands::queue`](../../prelude/struct.EntityCommands.html#method.queue "method bevy::prelude::EntityCommands::queue").

The command can be:

*   A custom struct that implements [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand").
*   A closure or function that matches the following signature:
    *   [`(EntityWorldMut)`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")
    *   [`(EntityWorldMut)`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") `->` [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")
*   A built-in command from the [`entity_command`](../system/entity_command/index.html "mod bevy::ecs::system::entity_command") module.

##### Example

```rust
use bevy_ecs::error::warn;

commands
    .spawn_empty()
    // Closures with this signature implement `EntityCommand`.
    .queue_handled(
        |entity: EntityWorldMut| -> Result {
            let value: usize = "100".parse()?;
            println!("Successfully parsed the value {} for entity {}", value, entity.id());
            Ok(())
        },
        warn
    );
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2011)

#### pub fn [queue\_silenced](#method.queue_silenced)( &mut self, command: impl [EntityCommand](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand"), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Pushes an [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") to the queue, which will get executed for the current [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

Unlike [`EntityCommands::queue_handled`](../../prelude/struct.EntityCommands.html#method.queue_handled "method bevy::prelude::EntityCommands::queue_handled"), this will completely ignore any errors that occur.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2050)

#### pub fn [retain](#method.retain)<B>(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes all components except the given [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") from the entity.

##### Example

```rust
#[derive(Component)]
struct Health(u32);
#[derive(Component)]
struct Strength(u32);
#[derive(Component)]
struct Defense(u32);

#[derive(Bundle)]
struct CombatBundle {
    health: Health,
    strength: Strength,
}

fn remove_combat_stats_system(mut commands: Commands, player: Res<PlayerEntity>) {
    commands
        .entity(player.entity)
        // You can retain a pre-defined Bundle of components,
        // with this removing only the Defense component.
        .retain::<CombatBundle>()
        // You can also retain only a single component.
        .retain::<Health>();
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2055)

#### pub fn [log\_components](#method.log_components)(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Logs the components of the entity at the [`info`](https://docs.rs/log/0.4.32/x86_64-unknown-linux-gnu/log/macro.info.html "macro log::info") level.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2060)

#### pub fn [commands](#method.commands)(&mut self) -> [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>

Returns the underlying [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2065)

#### pub fn [commands\_mut](#method.commands_mut)(&mut self) -> &mut [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'a, 'a>

Returns a mutable reference to the underlying [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2071)

#### pub fn [observe](#method.observe)<M>( &mut self, observer: impl [IntoEntityObserver](../observer/trait.IntoEntityObserver.html "trait bevy::ecs::observer::IntoEntityObserver")<M>, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Creates an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") watching for an [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") of type `E` whose [`EntityEvent::event_target`](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") targets this entity.

##### [Examples found in repository](#scraped-examples-12)[?](../../../scrape-examples-help.html)

examples/usage/context\_menu.rs ([lines 63-68](../../../src/context_menu/context_menu.rs.html#63-68))

```rust
60fn setup(mut commands: Commands) {
61    commands.spawn(Camera2d);
62
63    commands.spawn(background_and_button()).observe(
64        // any click bubbling up here should lead to closing any open menu
65        |_: On<Pointer<Press>>, mut commands: Commands| {
66            commands.trigger(CloseContextMenus);
67        },
68    );
69}
70
71fn on_trigger_close_menus(
72    _event: On<CloseContextMenus>,
73    mut commands: Commands,
74    menus: Query<Entity, With<ContextMenu>>,
75) {
76    for e in menus.iter() {
77        commands.entity(e).despawn();
78    }
79}
80
81fn on_trigger_menu(event: On<OpenContextMenu>, mut commands: Commands) {
82    commands.trigger(CloseContextMenus);
83
84    let pos = event.pos;
85
86    debug!("open context menu at: {pos}");
87
88    commands
89        .spawn((
90            Name::new("context menu"),
91            ContextMenu,
92            Node {
93                position_type: PositionType::Absolute,
94                left: px(pos.x),
95                top: px(pos.y),
96                flex_direction: FlexDirection::Column,
97                border_radius: BorderRadius::all(px(4)),
98                ..default()
99            },
100            BorderColor::all(Color::BLACK),
101            BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
102            children![
103                context_item("fuchsia", basic::FUCHSIA),
104                context_item("gray", basic::GRAY),
105                context_item("maroon", basic::MAROON),
106                context_item("purple", basic::PURPLE),
107                context_item("teal", basic::TEAL),
108            ],
109        ))
110        .observe(
111            |event: On<Pointer<Press>>,
112             menu_items: Query<&ContextMenuItem>,
113             mut clear_col: ResMut<ClearColor>,
114             mut commands: Commands| {
115                let target = event.original_event_target();
116
117                if let Ok(item) = menu_items.get(target) {
118                    clear_col.0 = item.0.into();
119                    commands.trigger(CloseContextMenus);
120                }
121            },
122        );
123}
```

Hide additional examples

examples/window/screenshot.rs ([line 27](../../../src/screenshot/screenshot.rs.html#27))

```rust
17fn screenshot_on_spacebar(
18    mut commands: Commands,
19    input: Res<ButtonInput<KeyCode>>,
20    mut counter: Local<u32>,
21) {
22    if input.just_pressed(KeyCode::Space) {
23        let path = format!("./screenshot-{}.png", *counter);
24        *counter += 1;
25        commands
26            .spawn(Screenshot::primary_window())
27            .observe(save_to_disk(path));
28    }
29}
```

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 59](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#59))

```rust
52fn setup_mesh_and_animation(mut commands: Commands, asset_server: Res<AssetServer>) {
53    // Spawn an entity with our components, and connect it to an observer that
54    // will trigger when the scene is loaded and spawned.
55    commands
56        .spawn(WorldAssetRoot(
57            asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH)),
58        ))
59        .observe(play_animation_when_ready);
60}
```

examples/ecs/observer\_propagation.rs ([line 30](../../../src/observer_propagation/observer_propagation.rs.html#30))

```rust
27fn setup(mut commands: Commands) {
28    commands
29        .spawn((Name::new("Goblin"), HitPoints(50)))
30        .observe(take_damage)
31        .with_children(|parent| {
32            parent
33                .spawn((Name::new("Helmet"), Armor(5)))
34                .observe(block_attack);
35            parent
36                .spawn((Name::new("Socks"), Armor(10)))
37                .observe(block_attack);
38            parent
39                .spawn((Name::new("Shirt"), Armor(15)))
40                .observe(block_attack);
41        });
42}
```

examples/ecs/hotpatching\_systems.rs ([line 78](../../../src/hotpatching_systems/hotpatching_systems.rs.html#78))

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

examples/3d/mixed\_lighting.rs ([lines 174-181](../../../src/mixed_lighting/mixed_lighting.rs.html#174-181))

```rust
166fn spawn_scene(commands: &mut Commands, asset_server: &AssetServer) {
167    commands
168        .spawn(WorldAssetRoot(
169            asset_server.load(
170                GltfAssetLabel::Scene(0)
171                    .from_asset("models/MixedLightingExample/MixedLightingExample.gltf"),
172            ),
173        ))
174        .observe(
175            |_: On<WorldInstanceReady>,
176             mut lighting_mode_changed_writer: MessageWriter<LightingModeChanged>| {
177                // When the scene loads, send a `LightingModeChanged` event so
178                // that we set up the lightmaps.
179                lighting_mode_changed_writer.write(LightingModeChanged);
180            },
181        );
182}
```

Additional examples can be found in:  

*   [examples/animation/morph\_targets.rs](../../../src/morph_targets/morph_targets.rs.html#45)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#92)
*   [examples/animation/animated\_mesh\_control.rs](../../../src/animated_mesh_control/animated_mesh_control.rs.html#135)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#341)
*   [examples/animation/animated\_mesh.rs](../../../src/animated_mesh/animated_mesh.rs.html#64)
*   [examples/picking/simple\_picking.rs](../../../src/simple_picking/simple_picking.rs.html#27)
*   [tests/window/desktop\_request\_redraw.rs](../../../src/desktop_request_redraw/desktop_request_redraw.rs.html#84-94)
*   [examples/picking/debug\_picking.rs](../../../src/debug_picking/debug_picking.rs.html#48)
*   [examples/remote/app\_under\_test.rs](../../../src/app_under_test/app_under_test.rs.html#99)
*   [examples/ecs/observers.rs](../../../src/observers/observers.rs.html#121)
*   [examples/ui/widgets/viewport\_node.rs](../../../src/viewport_node/viewport_node.rs.html#70)
*   [examples/ui/text/text\_background\_colors.rs](../../../src/text_background_colors/text_background_colors.rs.html#68-74)
*   [examples/picking/dragdrop\_picking.rs](../../../src/dragdrop_picking/dragdrop_picking.rs.html#66-72)
*   [examples/picking/sprite\_picking.rs](../../../src/sprite_picking/sprite_picking.rs.html#64)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../../src/many_morph_targets/many_morph_targets.rs.html#358)
*   [examples/gizmos/transform\_gizmo.rs](../../../src/transform_gizmo/transform_gizmo.rs.html#66)
*   [examples/shader/gpu\_readback.rs](../../../src/gpu_readback/gpu_readback.rs.html#103-108)
*   [examples/ui/text/system\_fonts.rs](../../../src/system_fonts/system_fonts.rs.html#81-94)
*   [examples/ui/ui\_target\_camera.rs](../../../src/ui_target_camera/ui_target_camera.rs.html#75-91)
*   [examples/ui/widgets/tab\_navigation.rs](../../../src/tab_navigation/tab_navigation.rs.html#82-87)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../../src/drag_to_scroll/drag_to_scroll.rs.html#41-52)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../../src/scroll/scroll.rs.html#175-181)
*   [examples/picking/mesh\_picking.rs](../../../src/mesh_picking/mesh_picking.rs.html#92)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#95)
*   [examples/ui/render\_ui\_to\_texture.rs](../../../src/render_ui_to_texture/render_ui_to_texture.rs.html#104-110)
*   [examples/ui/images/image\_node\_resizing.rs](../../../src/image_node_resizing/image_node_resizing.rs.html#104)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#187)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#60-65)
*   [examples/3d/contact\_shadows.rs](../../../src/contact_shadows/contact_shadows.rs.html#190-202)
*   [examples/ui/text/multiline\_text\_input.rs](../../../src/multiline_text_input/multiline_text_input.rs.html#84-106)
*   [examples/ui/styling/gradients.rs](../../../src/gradients/gradients.rs.html#204-209)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2112-2116)

#### pub fn [clone\_with\_opt\_out](#method.clone_with_opt_out)( &mut self, target: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), config: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [EntityClonerBuilder](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder")<'\_, [OptOut](../entity/struct.OptOut.html "struct bevy::ecs::entity::OptOut")\>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Clones parts of an entity (components, observers, etc.) onto another entity, configured through [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder").

The other entity will receive all the components of the original that implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") except those that are [denied](../entity/struct.EntityClonerBuilder.html#method.deny "method bevy::ecs::entity::EntityClonerBuilder::deny") in the `config`.

##### Panics

The command will panic when applied if the target entity does not exist.

##### Example

Configure through [`EntityClonerBuilder<OptOut>`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") as follows:

```rust
#[derive(Component, Clone)]
struct ComponentA(u32);
#[derive(Component, Clone)]
struct ComponentB(u32);

fn example_system(mut commands: Commands) {
    // Create an empty entity.
    let target = commands.spawn_empty().id();

    // Create a new entity and keep its EntityCommands.
    let mut entity = commands.spawn((ComponentA(10), ComponentB(20)));

    // Clone ComponentA but not ComponentB onto the target.
    entity.clone_with_opt_out(target, |builder| {
        builder.deny::<ComponentB>();
    });
}
```

See [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") for more options.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2157-2161)

#### pub fn [clone\_with\_opt\_in](#method.clone_with_opt_in)( &mut self, target: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), config: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [EntityClonerBuilder](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder")<'\_, [OptIn](../entity/struct.OptIn.html "struct bevy::ecs::entity::OptIn")\>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

Clones parts of an entity (components, observers, etc.) onto another entity, configured through [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder").

The other entity will receive only the components of the original that implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") and are [allowed](../entity/struct.EntityClonerBuilder.html#method.allow "method bevy::ecs::entity::EntityClonerBuilder::allow") in the `config`.

##### Panics

The command will panic when applied if the target entity does not exist.

##### Example

Configure through [`EntityClonerBuilder<OptIn>`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") as follows:

```rust
#[derive(Component, Clone)]
struct ComponentA(u32);
#[derive(Component, Clone)]
struct ComponentB(u32);

fn example_system(mut commands: Commands) {
    // Create an empty entity.
    let target = commands.spawn_empty().id();

    // Create a new entity and keep its EntityCommands.
    let mut entity = commands.spawn((ComponentA(10), ComponentB(20)));

    // Clone ComponentA but not ComponentB onto the target.
    entity.clone_with_opt_in(target, |builder| {
        builder.allow::<ComponentA>();
    });
}
```

See [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") for more options.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2196)

#### pub fn [clone\_and\_spawn](#method.clone_and_spawn)(&mut self) -> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Spawns a clone of this entity and returns the [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") of the clone.

The clone will receive all the components of the original that implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

To configure cloning behavior (such as only cloning certain components), use [`EntityCommands::clone_and_spawn_with_opt_out`](../../prelude/struct.EntityCommands.html#method.clone_and_spawn_with_opt_out "method bevy::prelude::EntityCommands::clone_and_spawn_with_opt_out")/ [`opt_out`](../../prelude/struct.EntityCommands.html#method.clone_and_spawn_with_opt_out "method bevy::prelude::EntityCommands::clone_and_spawn_with_opt_out").

##### Note

If the original entity does not exist when this command is applied, the returned entity will have no components.

##### Example

```rust
#[derive(Component, Clone)]
struct ComponentA(u32);
#[derive(Component, Clone)]
struct ComponentB(u32);

fn example_system(mut commands: Commands) {
    // Create a new entity and store its EntityCommands.
    let mut entity = commands.spawn((ComponentA(10), ComponentB(20)));

    // Create a clone of the entity.
    let mut entity_clone = entity.clone_and_spawn();
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2233-2236)

#### pub fn [clone\_and\_spawn\_with\_opt\_out](#method.clone_and_spawn_with_opt_out)( &mut self, config: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [EntityClonerBuilder](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder")<'\_, [OptOut](../entity/struct.OptOut.html "struct bevy::ecs::entity::OptOut")\>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Spawns a clone of this entity and allows configuring cloning behavior using [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder"), returning the [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") of the clone.

The clone will receive all the components of the original that implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") except those that are [denied](../entity/struct.EntityClonerBuilder.html#method.deny "method bevy::ecs::entity::EntityClonerBuilder::deny") in the `config`.

See the methods on [`EntityClonerBuilder<OptOut>`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") for more options.

##### Note

If the original entity does not exist when this command is applied, the returned entity will have no components.

##### Example

```rust
#[derive(Component, Clone)]
struct ComponentA(u32);
#[derive(Component, Clone)]
struct ComponentB(u32);

fn example_system(mut commands: Commands) {
    // Create a new entity and store its EntityCommands.
    let mut entity = commands.spawn((ComponentA(10), ComponentB(20)));

    // Create a clone of the entity with ComponentA but without ComponentB.
    let mut entity_clone = entity.clone_and_spawn_with_opt_out(|builder| {
        builder.deny::<ComponentB>();
    });
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2278-2281)

#### pub fn [clone\_and\_spawn\_with\_opt\_in](#method.clone_and_spawn_with_opt_in)( &mut self, config: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [EntityClonerBuilder](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder")<'\_, [OptIn](../entity/struct.OptIn.html "struct bevy::ecs::entity::OptIn")\>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Spawns a clone of this entity and allows configuring cloning behavior using [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder"), returning the [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") of the clone.

The clone will receive only the components of the original that implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") and are [allowed](../entity/struct.EntityClonerBuilder.html#method.allow "method bevy::ecs::entity::EntityClonerBuilder::allow") in the `config`.

See the methods on [`EntityClonerBuilder<OptIn>`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") for more options.

##### Note

If the original entity does not exist when this command is applied, the returned entity will have no components.

##### Example

```rust
#[derive(Component, Clone)]
struct ComponentA(u32);
#[derive(Component, Clone)]
struct ComponentB(u32);

fn example_system(mut commands: Commands) {
    // Create a new entity and store its EntityCommands.
    let mut entity = commands.spawn((ComponentA(10), ComponentB(20)));

    // Create a clone of the entity with ComponentA but without ComponentB.
    let mut entity_clone = entity.clone_and_spawn_with_opt_in(|builder| {
        builder.allow::<ComponentA>();
    });
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2298)

#### pub fn [clone\_components](#method.clone_components)<B>(&mut self, target: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Clones the specified components of this entity and inserts them into another entity.

Components can only be cloned if they implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

##### Panics

The command will panic when applied if the target entity does not exist.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2316)

#### pub fn [move\_components](#method.move_components)<B>(&mut self, target: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Moves the specified components of this entity into another entity.

Components with [`Ignore`](../component/enum.ComponentCloneBehavior.html#variant.Ignore "variant bevy::ecs::component::ComponentCloneBehavior::Ignore") clone behavior will not be moved, while components that have a [`Custom`](../component/enum.ComponentCloneBehavior.html#variant.Custom "variant bevy::ecs::component::ComponentCloneBehavior::Custom") clone behavior will be cloned using it and then removed from the source entity. All other components will be moved without any other special handling.

Note that this will trigger `on_remove` hooks/observers on this entity and `on_insert`/`on_add` hooks/observers on the target entity.

##### Panics

The command will panic when applied if the target entity does not exist.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2358-2361)

#### pub fn [trigger](#method.trigger)<'t, E>( &mut self, event\_fn: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> E, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

where E: [EntityEvent](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent"), <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'t>: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Passes the current entity into the given function, and triggers the [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") returned by that function.

##### Example

A surprising number of functions meet the trait bounds for `event_fn`:

```rust
#[derive(EntityEvent)]
struct Explode(Entity);

impl From<Entity> for Explode {
   fn from(entity: Entity) -> Self {
      Explode(entity)
   }
}


fn trigger_via_constructor(mut commands: Commands) {
    // The fact that `Explode` is a single-field tuple struct
    // ensures that `Explode(entity)` is a function that generates
    // an EntityEvent, meeting the trait bounds for `event_fn`.
    commands.spawn_empty().trigger(Explode);

}


fn trigger_via_from_trait(mut commands: Commands) {
    // This variant also works for events like `struct Explode { entity: Entity }`
    commands.spawn_empty().trigger(Explode::from);
}

fn trigger_via_closure(mut commands: Commands) {
    commands.spawn_empty().trigger(|entity| Explode(entity));
}
```

## Trait Implementations

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#31)

### impl [BuildChildrenTransformExt](../../prelude/trait.BuildChildrenTransformExt.html "trait bevy::prelude::BuildChildrenTransformExt") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#32)

#### fn [set\_parent\_in\_place](../../prelude/trait.BuildChildrenTransformExt.html#tymethod.set_parent_in_place)(&mut self, parent: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Change this entity’s parent while preserving this entity’s [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") by updating its [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform"). [Read more](../../prelude/trait.BuildChildrenTransformExt.html#tymethod.set_parent_in_place)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#38)

#### fn [remove\_parent\_in\_place](../../prelude/trait.BuildChildrenTransformExt.html#tymethod.remove_parent_in_place)(&mut self) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Make this entity parentless while preserving this entity’s [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") by updating its [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform") to be equal to its current [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform"). [Read more](../../prelude/trait.BuildChildrenTransformExt.html#tymethod.remove_parent_in_place)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#580)

### impl [EntityCommandsSceneExt](../../prelude/trait.EntityCommandsSceneExt.html "trait bevy::prelude::EntityCommandsSceneExt") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#581-584)

#### fn [queue\_spawn\_related\_scenes](../../prelude/trait.EntityCommandsSceneExt.html#tymethod.queue_spawn_related_scenes)<T>( &mut self, scenes: impl [SceneList](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

where T: [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"),

Spawns a [`SceneList`](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), where each entity is related to the current entity using [`RelationshipTarget::Relationship`](../../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "associated type bevy::prelude::RelationshipTarget::Relationship"). [Read more](../../prelude/trait.EntityCommandsSceneExt.html#tymethod.queue_spawn_related_scenes)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#591)

#### fn [apply\_scene](../../prelude/trait.EntityCommandsSceneExt.html#tymethod.apply_scene)<S>(&mut self, scene: S) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Applies the given [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") to the current entity as soon as [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") are applied. This will resolve the Scene (using [`Scene::resolve`](../../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve")). If that fails (for example, if there are dependencies that have not been loaded yet), it will log a [`SpawnSceneError`](../../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") as an error. If resolving the [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") is successful, the scene will be spawned. [Read more](../../prelude/trait.EntityCommandsSceneExt.html#tymethod.apply_scene)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#596)

#### fn [queue\_apply\_scene](../../prelude/trait.EntityCommandsSceneExt.html#tymethod.queue_apply_scene)<S>(&mut self, scene: S) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Queues the `scene` to be applied. This will evaluate the `scene`’s dependencies (via [`Scene::register_dependencies`](../../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](../../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error. [Read more](../../prelude/trait.EntityCommandsSceneExt.html#tymethod.queue_apply_scene)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#170)

### impl [ReflectCommandExt](../reflect/trait.ReflectCommandExt.html "trait bevy::ecs::reflect::ReflectCommandExt") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#171)

#### fn [insert\_reflect](../reflect/trait.ReflectCommandExt.html#tymethod.insert_reflect)( &mut self, component: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Adds the given boxed reflect component or bundle to the entity using the reflection data in [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry"). [Read more](../reflect/trait.ReflectCommandExt.html#tymethod.insert_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#177-180)

#### fn [insert\_reflect\_with\_registry](../reflect/trait.ReflectCommandExt.html#tymethod.insert_reflect_with_registry)<T>( &mut self, component: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry")\>,

Same as [`insert_reflect`](../reflect/trait.ReflectCommandExt.html#tymethod.insert_reflect "method bevy::ecs::reflect::ReflectCommandExt::insert_reflect"), but using the `T` resource as type registry instead of `AppTypeRegistry`. [Read more](../reflect/trait.ReflectCommandExt.html#tymethod.insert_reflect_with_registry)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#186)

#### fn [remove\_reflect](../reflect/trait.ReflectCommandExt.html#tymethod.remove_reflect)( &mut self, component\_type\_path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Removes from the entity the component or bundle with the given type path registered in [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry"). [Read more](../reflect/trait.ReflectCommandExt.html#tymethod.remove_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#193-196)

#### fn [remove\_reflect\_with\_registry](../reflect/trait.ReflectCommandExt.html#tymethod.remove_reflect_with_registry)<T>( &mut self, component\_type\_path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>, ) -> &mut [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry")\>,

Same as [`remove_reflect`](../reflect/trait.ReflectCommandExt.html#tymethod.remove_reflect "method bevy::ecs::reflect::ReflectCommandExt::remove_reflect"), but using the `T` resource as type registry instead of `AppTypeRegistry`.

## Auto Trait Implementations

### impl<'a> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

### impl<'a> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

### impl<'a> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

### impl<'a> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

### impl<'a> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

### impl<'a> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

### impl<'a> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'a>

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

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

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

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

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

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

### impl<T> [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

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

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}