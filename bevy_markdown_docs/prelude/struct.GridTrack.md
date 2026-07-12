[bevy](../index.html)::[prelude](index.html)

# Struct GridTrack 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1616)

```rust
pub struct GridTrack { /* private fields */ }
```

A [`GridTrack`](struct.GridTrack.html "struct bevy::prelude::GridTrack") is a Row or Column of a CSS Grid. This struct specifies what size the track should be. See below for the different “track sizing functions” you can specify.

## Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1621)

### impl [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1622)

#### pub const [DEFAULT](#associatedconstant.DEFAULT): [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1628)

#### pub fn [px](#method.px)<T>(value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track with a fixed pixel size

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 696](../../src/testbed_ui/ui.rs.html#696))

```rust
684    pub fn setup(mut commands: Commands) {
685        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Grid)));
686        // Top-level grid (app frame)
687        commands.spawn((
688            Node {
689                display: Display::Grid,
690                width: percent(100),
691                height: percent(100),
692                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
693                grid_template_rows: vec![
694                    GridTrack::auto(),
695                    GridTrack::flex(1.0),
696                    GridTrack::px(40.),
697                ],
698                ..default()
699            },
700            BackgroundColor(Color::WHITE),
701            DespawnOnExit(super::Scene::Grid),
702            children![
703                // Header
704                (
705                    Node {
706                        display: Display::Grid,
707                        grid_column: GridPlacement::span(2),
708                        padding: UiRect::all(px(40)),
709                        ..default()
710                    },
711                    BackgroundColor(RED.into()),
712                ),
713                // Main content grid (auto placed in row 2, column 1)
714                (
715                    Node {
716                        height: percent(100),
717                        aspect_ratio: Some(1.0),
718                        display: Display::Grid,
719                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
720                        grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
721                        row_gap: px(12),
722                        column_gap: px(12),
723                        ..default()
724                    },
725                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
726                    children![
727                        (Node::default(), BackgroundColor(ORANGE.into())),
728                        (Node::default(), BackgroundColor(BISQUE.into())),
729                        (Node::default(), BackgroundColor(BLUE.into())),
730                        (Node::default(), BackgroundColor(CRIMSON.into())),
731                        (Node::default(), BackgroundColor(AQUA.into())),
732                    ]
733                ),
734                // Right side bar (auto placed in row 2, column 2)
735                (Node::DEFAULT, BackgroundColor(BLACK.into())),
736            ],
737        ));
738    }
```

Hide additional examples

examples/ui/layout/grid.rs ([line 42](../../src/grid/grid.rs.html#42))

```rust
18fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
19    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
20    commands.spawn(Camera2d);
21
22    // Top-level grid (app frame)
23    commands
24        .spawn((
25            Node {
26                // Use the CSS Grid algorithm for laying out this node
27                display: Display::Grid,
28                // Make node fill the entirety of its parent (in this case the window)
29                width: percent(100),
30                height: percent(100),
31                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
32                //   - The first column will size to the size of its contents
33                //   - The second column will take up the remaining available space
34                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
35                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
36                //  - The first row will size to the size of its contents
37                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
38                //  - The third row will be exactly 20px high
39                grid_template_rows: vec![
40                    GridTrack::auto(),
41                    GridTrack::flex(1.0),
42                    GridTrack::px(20.),
43                ],
44                ..default()
45            },
46            BackgroundColor(Color::WHITE),
47        ))
48        .with_children(|builder| {
49            // Header
50            builder
51                .spawn(
52                    Node {
53                        display: Display::Grid,
54                        // Make this node span two grid columns so that it takes up the entire top tow
55                        grid_column: GridPlacement::span(2),
56                        padding: UiRect::all(px(6)),
57                        ..default()
58                    },
59                )
60                .with_children(|builder| {
61                    spawn_nested_text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example");
62                });
63
64            // Main content grid (auto placed in row 2, column 1)
65            builder
66                .spawn((
67                    Node {
68                        // Make the height of the node fill its parent
69                        height: percent(100),
70                        // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
71                        // As the height is set explicitly, this means the width will adjust to match the height
72                        aspect_ratio: Some(1.0),
73                        // Use grid layout for this node
74                        display: Display::Grid,
75                        // Add 24px of padding around the grid
76                        padding: UiRect::all(px(24)),
77                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
78                        // This creates 4 exactly evenly sized columns
79                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
80                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
81                        // This creates 4 exactly evenly sized rows
82                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
83                        // Set a 12px gap/gutter between rows and columns
84                        row_gap: px(12),
85                        column_gap: px(12),
86                        ..default()
87                    },
88                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
89                ))
90                .with_children(|builder| {
91                    // Note there is no need to specify the position for each grid item. Grid items that are
92                    // not given an explicit position will be automatically positioned into the next available
93                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
94                    // style property.
95
96                    item_rect(builder, ORANGE);
97                    item_rect(builder, BISQUE);
98                    item_rect(builder, BLUE);
99                    item_rect(builder, CRIMSON);
100                    item_rect(builder, AQUA);
101                    item_rect(builder, ORANGE_RED);
102                    item_rect(builder, DARK_GREEN);
103                    item_rect(builder, FUCHSIA);
104                    item_rect(builder, TEAL);
105                    item_rect(builder, ALICE_BLUE);
106                    item_rect(builder, CRIMSON);
107                    item_rect(builder, ANTIQUE_WHITE);
108                    item_rect(builder, YELLOW);
109                    item_rect(builder, DEEP_PINK);
110                    item_rect(builder, YELLOW_GREEN);
111                    item_rect(builder, SALMON);
112                });
113
114            // Right side bar (auto placed in row 2, column 2)
115            builder
116                .spawn((
117                    Node {
118                        display: Display::Grid,
119                        // Align content towards the start (top) in the vertical axis
120                        align_items: AlignItems::Start,
121                        // Align content towards the center in the horizontal axis
122                        justify_items: JustifyItems::Center,
123                        // Add 10px padding
124                        padding: UiRect::all(px(10)),
125                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
126                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
127                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
128                        // Add a 10px gap between rows
129                        row_gap: px(10),
130                        ..default()
131                    },
132                    BackgroundColor(BLACK.into()),
133                ))
134                .with_children(|builder| {
135                    builder.spawn((Text::new("Sidebar"),
136                        TextFont::from(font.clone()),
137                    ));
138                    builder.spawn((Text::new("A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely."),
139                        TextFont {
140                            font: font.clone().into(),
141                            font_size: FontSize::Px(13.0),
142                            ..default()
143                        },
144                    ));
145                    builder.spawn(Node::default());
146                });
147
148            // Footer / status bar
149            builder.spawn((
150                Node {
151                    // Make this node span two grid column so that it takes up the entire bottom row
152                    grid_column: GridPlacement::span(2),
153                    ..default()
154                },
155                BackgroundColor(WHITE.into()),
156            ));
157
158            // Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
159            builder.spawn((
160                Node {
161                    position_type: PositionType::Absolute,
162                    margin: UiRect {
163                        top: px(100),
164                        bottom: auto(),
165                        left: auto(),
166                        right: auto(),
167                    },
168                    width: percent(60),
169                    height: px(300),
170                    max_width: px(600),
171                    ..default()
172                },
173                Visibility::Hidden,
174                BackgroundColor(Color::WHITE.with_alpha(0.8)),
175            ));
176        });
177}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1637)

#### pub fn [percent](#method.percent)<T>(value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track with a percentage size

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1648)

#### pub fn [fr](#method.fr)<T>(value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track with an `fr` size. Note that this will give the track a content-based minimum size. Usually you are best off using `GridTrack::flex` instead which uses a zero minimum size.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/ui/layout/grid.rs ([line 127](../../src/grid/grid.rs.html#127))

```rust
18fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
19    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
20    commands.spawn(Camera2d);
21
22    // Top-level grid (app frame)
23    commands
24        .spawn((
25            Node {
26                // Use the CSS Grid algorithm for laying out this node
27                display: Display::Grid,
28                // Make node fill the entirety of its parent (in this case the window)
29                width: percent(100),
30                height: percent(100),
31                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
32                //   - The first column will size to the size of its contents
33                //   - The second column will take up the remaining available space
34                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
35                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
36                //  - The first row will size to the size of its contents
37                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
38                //  - The third row will be exactly 20px high
39                grid_template_rows: vec![
40                    GridTrack::auto(),
41                    GridTrack::flex(1.0),
42                    GridTrack::px(20.),
43                ],
44                ..default()
45            },
46            BackgroundColor(Color::WHITE),
47        ))
48        .with_children(|builder| {
49            // Header
50            builder
51                .spawn(
52                    Node {
53                        display: Display::Grid,
54                        // Make this node span two grid columns so that it takes up the entire top tow
55                        grid_column: GridPlacement::span(2),
56                        padding: UiRect::all(px(6)),
57                        ..default()
58                    },
59                )
60                .with_children(|builder| {
61                    spawn_nested_text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example");
62                });
63
64            // Main content grid (auto placed in row 2, column 1)
65            builder
66                .spawn((
67                    Node {
68                        // Make the height of the node fill its parent
69                        height: percent(100),
70                        // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
71                        // As the height is set explicitly, this means the width will adjust to match the height
72                        aspect_ratio: Some(1.0),
73                        // Use grid layout for this node
74                        display: Display::Grid,
75                        // Add 24px of padding around the grid
76                        padding: UiRect::all(px(24)),
77                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
78                        // This creates 4 exactly evenly sized columns
79                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
80                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
81                        // This creates 4 exactly evenly sized rows
82                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
83                        // Set a 12px gap/gutter between rows and columns
84                        row_gap: px(12),
85                        column_gap: px(12),
86                        ..default()
87                    },
88                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
89                ))
90                .with_children(|builder| {
91                    // Note there is no need to specify the position for each grid item. Grid items that are
92                    // not given an explicit position will be automatically positioned into the next available
93                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
94                    // style property.
95
96                    item_rect(builder, ORANGE);
97                    item_rect(builder, BISQUE);
98                    item_rect(builder, BLUE);
99                    item_rect(builder, CRIMSON);
100                    item_rect(builder, AQUA);
101                    item_rect(builder, ORANGE_RED);
102                    item_rect(builder, DARK_GREEN);
103                    item_rect(builder, FUCHSIA);
104                    item_rect(builder, TEAL);
105                    item_rect(builder, ALICE_BLUE);
106                    item_rect(builder, CRIMSON);
107                    item_rect(builder, ANTIQUE_WHITE);
108                    item_rect(builder, YELLOW);
109                    item_rect(builder, DEEP_PINK);
110                    item_rect(builder, YELLOW_GREEN);
111                    item_rect(builder, SALMON);
112                });
113
114            // Right side bar (auto placed in row 2, column 2)
115            builder
116                .spawn((
117                    Node {
118                        display: Display::Grid,
119                        // Align content towards the start (top) in the vertical axis
120                        align_items: AlignItems::Start,
121                        // Align content towards the center in the horizontal axis
122                        justify_items: JustifyItems::Center,
123                        // Add 10px padding
124                        padding: UiRect::all(px(10)),
125                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
126                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
127                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
128                        // Add a 10px gap between rows
129                        row_gap: px(10),
130                        ..default()
131                    },
132                    BackgroundColor(BLACK.into()),
133                ))
134                .with_children(|builder| {
135                    builder.spawn((Text::new("Sidebar"),
136                        TextFont::from(font.clone()),
137                    ));
138                    builder.spawn((Text::new("A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely."),
139                        TextFont {
140                            font: font.clone().into(),
141                            font_size: FontSize::Px(13.0),
142                            ..default()
143                        },
144                    ));
145                    builder.spawn(Node::default());
146                });
147
148            // Footer / status bar
149            builder.spawn((
150                Node {
151                    // Make this node span two grid column so that it takes up the entire bottom row
152                    grid_column: GridPlacement::span(2),
153                    ..default()
154                },
155                BackgroundColor(WHITE.into()),
156            ));
157
158            // Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
159            builder.spawn((
160                Node {
161                    position_type: PositionType::Absolute,
162                    margin: UiRect {
163                        top: px(100),
164                        bottom: auto(),
165                        left: auto(),
166                        right: auto(),
167                    },
168                    width: percent(60),
169                    height: px(300),
170                    max_width: px(600),
171                    ..default()
172                },
173                Visibility::Hidden,
174                BackgroundColor(Color::WHITE.with_alpha(0.8)),
175            ));
176        });
177}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1657)

#### pub fn [flex](#method.flex)<T>(value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track with a `minmax(0, Nfr)` size.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/ui/text/system\_fonts.rs ([line 62](../../src/system_fonts/system_fonts.rs.html#62))

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

Hide additional examples

examples/ui/images/image\_node\_resizing.rs ([line 77](../../src/image_node_resizing/image_node_resizing.rs.html#77))

```rust
62fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
63    let image_handle = asset_server.load("branding/icon.png");
64    let full_text = format!(
65        "{}height : {}%, width : {}%",
66        TEXT_PREFIX, IMAGE_GROUP_BOX_INIT_HEIGHT, IMAGE_GROUP_BOX_INIT_WIDTH,
67    );
68
69    commands.spawn(Camera2d);
70
71    let container = commands
72        .spawn((
73            Node {
74                display: Display::Grid,
75                width: percent(100),
76                height: percent(100),
77                grid_template_rows: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
78                ..default()
79            },
80            BackgroundColor(Color::WHITE),
81        ))
82        .id();
83
84    // Keyboard Text
85    commands
86        .spawn((
87            TextData {
88                height: IMAGE_GROUP_BOX_INIT_HEIGHT,
89                width: IMAGE_GROUP_BOX_INIT_WIDTH,
90            },
91            Text::new(full_text),
92            TextColor::BLACK,
93            Node {
94                grid_row: GridPlacement::span(1),
95                padding: px(6).all(),
96                ..default()
97            },
98            UiDebugOptions {
99                enabled: false,
100                ..default()
101            },
102            ChildOf(container),
103        ))
104        .observe(update_text);
105
106    commands
107        .spawn((
108            Node {
109                display: Display::Flex,
110                grid_row: GridPlacement::span(1),
111                flex_direction: FlexDirection::Column,
112                justify_content: JustifyContent::SpaceAround,
113                padding: px(10.).all(),
114                ..default()
115            },
116            BackgroundColor(Color::BLACK),
117            ChildOf(container),
118        ))
119        .with_children(|builder| {
120            // `NodeImageMode::Auto` will resize the image automatically by taking the size of the source image and applying any layout constraints.
121            builder
122                .spawn((
123                    ImageGroup,
124                    Node {
125                        display: Display::Flex,
126                        justify_content: JustifyContent::Start,
127                        width: percent(IMAGE_GROUP_BOX_INIT_WIDTH),
128                        height: percent(IMAGE_GROUP_BOX_INIT_HEIGHT),
129                        ..default()
130                    },
131                    BackgroundColor(Color::from(tailwind::BLUE_100)),
132                ))
133                .with_children(|parent| {
134                    for _ in 0..4 {
135                        // child node will apply Flex layout
136                        parent.spawn((
137                            Node::default(),
138                            ImageNode {
139                                image: image_handle.clone(),
140                                image_mode: NodeImageMode::Auto,
141                                ..default()
142                            },
143                        ));
144                    }
145                });
146            // `NodeImageMode::Stretch` will resize the image to match the size of the `Node` component
147            builder
148                .spawn((
149                    ImageGroup,
150                    Node {
151                        display: Display::Flex,
152                        justify_content: JustifyContent::Start,
153                        width: percent(IMAGE_GROUP_BOX_INIT_WIDTH),
154                        height: percent(IMAGE_GROUP_BOX_INIT_HEIGHT),
155                        ..default()
156                    },
157                    BackgroundColor(Color::from(tailwind::BLUE_100)),
158                ))
159                .with_children(|parent| {
160                    for width in [10., 20., 30., 40.] {
161                        parent.spawn((
162                            Node {
163                                height: percent(100),
164                                width: percent(width),
165                                ..default()
166                            },
167                            ImageNode {
168                                image: image_handle.clone(),
169                                image_mode: NodeImageMode::Stretch,
170                                ..default()
171                            },
172                        ));
173                    }
174                });
175        });
176}
```

examples/ui/layout/grid.rs ([line 34](../../src/grid/grid.rs.html#34))

```rust
18fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
19    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
20    commands.spawn(Camera2d);
21
22    // Top-level grid (app frame)
23    commands
24        .spawn((
25            Node {
26                // Use the CSS Grid algorithm for laying out this node
27                display: Display::Grid,
28                // Make node fill the entirety of its parent (in this case the window)
29                width: percent(100),
30                height: percent(100),
31                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
32                //   - The first column will size to the size of its contents
33                //   - The second column will take up the remaining available space
34                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
35                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
36                //  - The first row will size to the size of its contents
37                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
38                //  - The third row will be exactly 20px high
39                grid_template_rows: vec![
40                    GridTrack::auto(),
41                    GridTrack::flex(1.0),
42                    GridTrack::px(20.),
43                ],
44                ..default()
45            },
46            BackgroundColor(Color::WHITE),
47        ))
48        .with_children(|builder| {
49            // Header
50            builder
51                .spawn(
52                    Node {
53                        display: Display::Grid,
54                        // Make this node span two grid columns so that it takes up the entire top tow
55                        grid_column: GridPlacement::span(2),
56                        padding: UiRect::all(px(6)),
57                        ..default()
58                    },
59                )
60                .with_children(|builder| {
61                    spawn_nested_text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example");
62                });
63
64            // Main content grid (auto placed in row 2, column 1)
65            builder
66                .spawn((
67                    Node {
68                        // Make the height of the node fill its parent
69                        height: percent(100),
70                        // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
71                        // As the height is set explicitly, this means the width will adjust to match the height
72                        aspect_ratio: Some(1.0),
73                        // Use grid layout for this node
74                        display: Display::Grid,
75                        // Add 24px of padding around the grid
76                        padding: UiRect::all(px(24)),
77                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
78                        // This creates 4 exactly evenly sized columns
79                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
80                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
81                        // This creates 4 exactly evenly sized rows
82                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
83                        // Set a 12px gap/gutter between rows and columns
84                        row_gap: px(12),
85                        column_gap: px(12),
86                        ..default()
87                    },
88                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
89                ))
90                .with_children(|builder| {
91                    // Note there is no need to specify the position for each grid item. Grid items that are
92                    // not given an explicit position will be automatically positioned into the next available
93                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
94                    // style property.
95
96                    item_rect(builder, ORANGE);
97                    item_rect(builder, BISQUE);
98                    item_rect(builder, BLUE);
99                    item_rect(builder, CRIMSON);
100                    item_rect(builder, AQUA);
101                    item_rect(builder, ORANGE_RED);
102                    item_rect(builder, DARK_GREEN);
103                    item_rect(builder, FUCHSIA);
104                    item_rect(builder, TEAL);
105                    item_rect(builder, ALICE_BLUE);
106                    item_rect(builder, CRIMSON);
107                    item_rect(builder, ANTIQUE_WHITE);
108                    item_rect(builder, YELLOW);
109                    item_rect(builder, DEEP_PINK);
110                    item_rect(builder, YELLOW_GREEN);
111                    item_rect(builder, SALMON);
112                });
113
114            // Right side bar (auto placed in row 2, column 2)
115            builder
116                .spawn((
117                    Node {
118                        display: Display::Grid,
119                        // Align content towards the start (top) in the vertical axis
120                        align_items: AlignItems::Start,
121                        // Align content towards the center in the horizontal axis
122                        justify_items: JustifyItems::Center,
123                        // Add 10px padding
124                        padding: UiRect::all(px(10)),
125                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
126                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
127                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
128                        // Add a 10px gap between rows
129                        row_gap: px(10),
130                        ..default()
131                    },
132                    BackgroundColor(BLACK.into()),
133                ))
134                .with_children(|builder| {
135                    builder.spawn((Text::new("Sidebar"),
136                        TextFont::from(font.clone()),
137                    ));
138                    builder.spawn((Text::new("A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely."),
139                        TextFont {
140                            font: font.clone().into(),
141                            font_size: FontSize::Px(13.0),
142                            ..default()
143                        },
144                    ));
145                    builder.spawn(Node::default());
146                });
147
148            // Footer / status bar
149            builder.spawn((
150                Node {
151                    // Make this node span two grid column so that it takes up the entire bottom row
152                    grid_column: GridPlacement::span(2),
153                    ..default()
154                },
155                BackgroundColor(WHITE.into()),
156            ));
157
158            // Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
159            builder.spawn((
160                Node {
161                    position_type: PositionType::Absolute,
162                    margin: UiRect {
163                        top: px(100),
164                        bottom: auto(),
165                        left: auto(),
166                        right: auto(),
167                    },
168                    width: percent(60),
169                    height: px(300),
170                    max_width: px(600),
171                    ..default()
172                },
173                Visibility::Hidden,
174                BackgroundColor(Color::WHITE.with_alpha(0.8)),
175            ));
176        });
177}
```

examples/testbed/ui.rs ([line 232](../../src/testbed_ui/ui.rs.html#232))

```rust
209    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
210        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Text)));
211
212        let mut container = commands.spawn((
213            Node {
214                flex_direction: FlexDirection::Column,
215                ..default()
216            },
217            DespawnOnExit(super::Scene::Text),
218        ));
219
220        container.with_child((
221            Text::new("Hello World."),
222            TextFont {
223                font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
224                font_size: FontSize::Px(200.),
225                ..default()
226            },
227        ));
228
229        container.with_children(|builder| {
230            let mut grid = builder.spawn(Node {
231                display: Display::Grid,
232                grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
233                padding: UiRect::horizontal(px(5.)),
234                ..default()
235            });
236
237            grid.with_children(|grid| {
238                for hinting in [FontHinting::Enabled, FontHinting::Disabled] {
239                    let mut content = grid.spawn(Node {
240                        flex_direction: FlexDirection::Column,
241                        row_gap: px(5.),
242                        ..default()
243                    });
244
245                    content.with_child((
246                        Text::new(format!("FontHinting::{:?}", hinting)),
247                        TextFont {
248                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
249                            ..default()
250                        },
251                        hinting,
252                    ));
253
254                    content.with_child((
255                        Text::new("white "),
256                        TextFont {
257                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
258                            ..default()
259                        },
260                        hinting,
261                        children![
262                            (TextSpan::new("red "), TextColor(RED.into()),),
263                            (TextSpan::new("green "), TextColor(GREEN.into()),),
264                            (TextSpan::new("blue "), TextColor(BLUE.into()),),
265                            (
266                                TextSpan::new("black"),
267                                TextColor(Color::BLACK),
268                                TextFont {
269                                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
270                                    ..default()
271                                },
272                                TextBackgroundColor(Color::WHITE)
273                            ),
274                        ],
275                    ));
276
277                    content.with_child((
278                        Text::new(""),
279                        TextFont {
280                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
281                            ..default()
282                        },
283                        hinting,
284                        children![
285                            (
286                                TextSpan::new("white "),
287                                TextFont {
288                                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
289                                    ..default()
290                                }
291                            ),
292                            (TextSpan::new("red "), TextColor(RED.into()),),
293                            (TextSpan::new("green "), TextColor(GREEN.into()),),
294                            (TextSpan::new("blue "), TextColor(BLUE.into()),),
295                            (
296                                TextSpan::new("black"),
297                                TextColor(Color::BLACK),
298                                TextFont {
299                                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
300                                    ..default()
301                                },
302                                TextBackgroundColor(Color::WHITE)
303                            ),
304                        ],
305                    ));
306
307                    content.with_child((
308                        Text::new(""),
309                        TextFont {
310                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
311                            ..default()
312                        },
313                        hinting,
314                        children![
315                            (TextSpan::new(""), TextColor(YELLOW.into()),),
316                            TextSpan::new(""),
317                            (
318                                TextSpan::new("white "),
319                                TextFont {
320                                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
321                                    ..default()
322                                }
323                            ),
324                            TextSpan::new(""),
325                            (TextSpan::new("red "), TextColor(RED.into()),),
326                            TextSpan::new(""),
327                            TextSpan::new(""),
328                            (TextSpan::new("green "), TextColor(GREEN.into()),),
329                            (TextSpan::new(""), TextColor(YELLOW.into()),),
330                            (TextSpan::new("blue "), TextColor(BLUE.into()),),
331                            TextSpan::new(""),
332                            (TextSpan::new(""), TextColor(YELLOW.into()),),
333                            (
334                                TextSpan::new("black"),
335                                TextColor(Color::BLACK),
336                                TextFont {
337                                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
338                                    ..default()
339                                },
340                                TextBackgroundColor(Color::WHITE)
341                            ),
342                            TextSpan::new(""),
343                        ],
344                    ));
345
346                    content.with_child((
347                        hinting,
348                        Text::new("FiraSans_"),
349                        TextFont {
350                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
351                            font_size: FontSize::Px(25.),
352                            ..default()
353                        },
354                        children![
355                            (
356                                TextSpan::new("MonaSans_"),
357                                TextFont {
358                                    font: asset_server
359                                        .load("fonts/MonaSans-VariableFont.ttf")
360                                        .into(),
361                                    font_size: FontSize::Px(25.),
362                                    ..default()
363                                }
364                            ),
365                            (
366                                TextSpan::new("EBGaramond_"),
367                                TextFont {
368                                    font: asset_server
369                                        .load("fonts/EBGaramond12-Regular.otf")
370                                        .into(),
371                                    font_size: FontSize::Px(25.),
372                                    ..default()
373                                },
374                            ),
375                            (
376                                TextSpan::new("FiraMono"),
377                                TextFont {
378                                    font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
379                                    font_size: FontSize::Px(25.),
380                                    ..default()
381                                },
382                            ),
383                        ],
384                    ));
385
386                    content.with_child((
387                        hinting,
388                        Text::new("FiraSans "),
389                        TextFont {
390                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
391                            font_size: FontSize::Px(25.),
392                            ..default()
393                        },
394                        children![
395                            (
396                                TextSpan::new("MonaSans "),
397                                TextFont {
398                                    font: asset_server
399                                        .load("fonts/MonaSans-VariableFont.ttf")
400                                        .into(),
401                                    font_size: FontSize::Px(25.),
402                                    ..default()
403                                }
404                            ),
405                            (
406                                TextSpan::new("EBGaramond "),
407                                TextFont {
408                                    font: asset_server
409                                        .load("fonts/EBGaramond12-Regular.otf")
410                                        .into(),
411                                    font_size: FontSize::Px(25.),
412                                    ..default()
413                                },
414                            ),
415                            (
416                                TextSpan::new("FiraMono"),
417                                TextFont {
418                                    font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
419                                    font_size: FontSize::Px(25.),
420                                    ..default()
421                                },
422                            ),
423                        ],
424                    ));
425
426                    content.with_child((
427                        hinting,
428                        Text::new("FiraSans "),
429                        TextFont {
430                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
431                            font_size: FontSize::Px(25.),
432                            ..default()
433                        },
434                        children![
435                            (
436                                TextSpan::new("MonaSans_"),
437                                TextFont {
438                                    font: asset_server
439                                        .load("fonts/MonaSans-VariableFont.ttf")
440                                        .into(),
441                                    font_size: FontSize::Px(25.),
442                                    ..default()
443                                }
444                            ),
445                            (
446                                TextSpan::new("EBGaramond "),
447                                TextFont {
448                                    font: asset_server
449                                        .load("fonts/EBGaramond12-Regular.otf")
450                                        .into(),
451                                    font_size: FontSize::Px(25.),
452                                    ..default()
453                                },
454                            ),
455                            (
456                                TextSpan::new("FiraMono"),
457                                TextFont {
458                                    font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
459                                    font_size: FontSize::Px(25.),
460                                    ..default()
461                                },
462                            ),
463                        ],
464                    ));
465
466                    content.with_child((
467                        hinting,
468                        Text::new("FiraSans"),
469                        TextFont {
470                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
471                            font_size: FontSize::Px(25.),
472                            ..default()
473                        },
474                        children![
475                            TextSpan::new(" "),
476                            (
477                                TextSpan::new("MonaSans"),
478                                TextFont {
479                                    font: asset_server
480                                        .load("fonts/MonaSans-VariableFont.ttf")
481                                        .into(),
482                                    font_size: FontSize::Px(25.),
483                                    ..default()
484                                }
485                            ),
486                            TextSpan::new(" "),
487                            (
488                                TextSpan::new("EBGaramond"),
489                                TextFont {
490                                    font: asset_server
491                                        .load("fonts/EBGaramond12-Regular.otf")
492                                        .into(),
493                                    font_size: FontSize::Px(25.),
494                                    ..default()
495                                },
496                            ),
497                            TextSpan::new(" "),
498                            (
499                                TextSpan::new("FiraMono"),
500                                TextFont {
501                                    font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
502                                    font_size: FontSize::Px(25.),
503                                    ..default()
504                                },
505                            ),
506                        ],
507                    ));
508
509                    content.with_child((
510                        hinting,
511                        Text::new("Fira Sans_"),
512                        TextFont {
513                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
514                            font_size: FontSize::Px(25.),
515                            ..default()
516                        },
517                        children![
518                            (
519                                TextSpan::new("Mona Sans_"),
520                                TextFont {
521                                    font: asset_server
522                                        .load("fonts/MonaSans-VariableFont.ttf")
523                                        .into(),
524                                    font_size: FontSize::Px(25.),
525                                    ..default()
526                                }
527                            ),
528                            (
529                                TextSpan::new("EB Garamond_"),
530                                TextFont {
531                                    font: asset_server
532                                        .load("fonts/EBGaramond12-Regular.otf")
533                                        .into(),
534                                    font_size: FontSize::Px(25.),
535                                    ..default()
536                                },
537                            ),
538                            (
539                                TextSpan::new("Fira Mono"),
540                                TextFont {
541                                    font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
542                                    font_size: FontSize::Px(25.),
543                                    ..default()
544                                },
545                            ),
546                        ],
547                    ));
548
549                    content.with_child((
550                        hinting,
551                        Text::new("FontWeight(100)_"),
552                        TextFont {
553                            font: "Mona Sans".into(),
554                            font_size: FontSize::Px(25.),
555                            weight: FontWeight(100),
556                            ..default()
557                        },
558                        children![
559                            (
560                                TextSpan::new("FontWeight(500)_"),
561                                TextFont {
562                                    font: "Mona Sans".into(),
563                                    font_size: FontSize::Px(25.),
564                                    weight: FontWeight(500),
565                                    ..default()
566                                }
567                            ),
568                            (
569                                TextSpan::new("FontWeight(900)"),
570                                TextFont {
571                                    font: "Mona Sans".into(),
572                                    font_size: FontSize::Px(25.),
573                                    weight: FontWeight(900),
574                                    ..default()
575                                },
576                            ),
577                        ],
578                    ));
579
580                    content.with_child((
581                        hinting,
582                        Text::new("FiraSans_"),
583                        TextFont {
584                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
585                            font_size: FontSize::Px(25.),
586                            weight: FontWeight(900),
587                            ..default()
588                        },
589                        children![
590                            (
591                                TextSpan::new("MonaSans_"),
592                                TextFont {
593                                    font: asset_server
594                                        .load("fonts/MonaSans-VariableFont.ttf")
595                                        .into(),
596                                    font_size: FontSize::Px(25.),
597                                    weight: FontWeight(700),
598                                    ..default()
599                                }
600                            ),
601                            (
602                                TextSpan::new("EBGaramond_"),
603                                TextFont {
604                                    font: asset_server
605                                        .load("fonts/EBGaramond12-Regular.otf")
606                                        .into(),
607                                    font_size: FontSize::Px(25.),
608                                    weight: FontWeight(500),
609                                    ..default()
610                                },
611                            ),
612                            (
613                                TextSpan::new("FiraMono"),
614                                TextFont {
615                                    font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
616                                    font_size: FontSize::Px(25.),
617                                    weight: FontWeight(300),
618                                    ..default()
619                                },
620                            ),
621                        ],
622                    ));
623
624                    content.with_child((
625                        hinting,
626                        Text::new("FiraSans\t"),
627                        TextFont {
628                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
629                            font_size: FontSize::Px(25.),
630                            ..default()
631                        },
632                        children![
633                            (
634                                TextSpan::new("MonaSans\t"),
635                                TextFont {
636                                    font: asset_server
637                                        .load("fonts/MonaSans-VariableFont.ttf")
638                                        .into(),
639                                    font_size: FontSize::Px(25.),
640                                    ..default()
641                                }
642                            ),
643                            (
644                                TextSpan::new("EBGaramond\t"),
645                                TextFont {
646                                    font: asset_server
647                                        .load("fonts/EBGaramond12-Regular.otf")
648                                        .into(),
649                                    font_size: FontSize::Px(25.),
650                                    ..default()
651                                },
652                            ),
653                            (
654                                TextSpan::new("FiraMono"),
655                                TextFont {
656                                    font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
657                                    font_size: FontSize::Px(25.),
658                                    ..default()
659                                },
660                            ),
661                        ],
662                    ));
663
664                    for font_smoothing in [FontSmoothing::AntiAliased, FontSmoothing::None] {
665                        content.with_child((
666                            Text::new(format!("FontSmoothing::{:?}", font_smoothing)),
667                            TextFont {
668                                font: asset_server.load("fonts/MonaSans-VariableFont.ttf").into(),
669                                font_size: FontSize::Px(25.),
670                                font_smoothing,
671                                ..default()
672                            },
673                        ));
674                    }
675                }
676            });
677        });
678    }
679}
680
681mod grid {
682    use bevy::{color::palettes::css::*, prelude::*};
683
684    pub fn setup(mut commands: Commands) {
685        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Grid)));
686        // Top-level grid (app frame)
687        commands.spawn((
688            Node {
689                display: Display::Grid,
690                width: percent(100),
691                height: percent(100),
692                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
693                grid_template_rows: vec![
694                    GridTrack::auto(),
695                    GridTrack::flex(1.0),
696                    GridTrack::px(40.),
697                ],
698                ..default()
699            },
700            BackgroundColor(Color::WHITE),
701            DespawnOnExit(super::Scene::Grid),
702            children![
703                // Header
704                (
705                    Node {
706                        display: Display::Grid,
707                        grid_column: GridPlacement::span(2),
708                        padding: UiRect::all(px(40)),
709                        ..default()
710                    },
711                    BackgroundColor(RED.into()),
712                ),
713                // Main content grid (auto placed in row 2, column 1)
714                (
715                    Node {
716                        height: percent(100),
717                        aspect_ratio: Some(1.0),
718                        display: Display::Grid,
719                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
720                        grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
721                        row_gap: px(12),
722                        column_gap: px(12),
723                        ..default()
724                    },
725                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
726                    children![
727                        (Node::default(), BackgroundColor(ORANGE.into())),
728                        (Node::default(), BackgroundColor(BISQUE.into())),
729                        (Node::default(), BackgroundColor(BLUE.into())),
730                        (Node::default(), BackgroundColor(CRIMSON.into())),
731                        (Node::default(), BackgroundColor(AQUA.into())),
732                    ]
733                ),
734                // Right side bar (auto placed in row 2, column 2)
735                (Node::DEFAULT, BackgroundColor(BLACK.into())),
736            ],
737        ));
738    }
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1666)

#### pub fn [auto](#method.auto)<T>() -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track which is automatically sized to fit its contents.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 694](../../src/testbed_ui/ui.rs.html#694))

```rust
684    pub fn setup(mut commands: Commands) {
685        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Grid)));
686        // Top-level grid (app frame)
687        commands.spawn((
688            Node {
689                display: Display::Grid,
690                width: percent(100),
691                height: percent(100),
692                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
693                grid_template_rows: vec![
694                    GridTrack::auto(),
695                    GridTrack::flex(1.0),
696                    GridTrack::px(40.),
697                ],
698                ..default()
699            },
700            BackgroundColor(Color::WHITE),
701            DespawnOnExit(super::Scene::Grid),
702            children![
703                // Header
704                (
705                    Node {
706                        display: Display::Grid,
707                        grid_column: GridPlacement::span(2),
708                        padding: UiRect::all(px(40)),
709                        ..default()
710                    },
711                    BackgroundColor(RED.into()),
712                ),
713                // Main content grid (auto placed in row 2, column 1)
714                (
715                    Node {
716                        height: percent(100),
717                        aspect_ratio: Some(1.0),
718                        display: Display::Grid,
719                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
720                        grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
721                        row_gap: px(12),
722                        column_gap: px(12),
723                        ..default()
724                    },
725                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
726                    children![
727                        (Node::default(), BackgroundColor(ORANGE.into())),
728                        (Node::default(), BackgroundColor(BISQUE.into())),
729                        (Node::default(), BackgroundColor(BLUE.into())),
730                        (Node::default(), BackgroundColor(CRIMSON.into())),
731                        (Node::default(), BackgroundColor(AQUA.into())),
732                    ]
733                ),
734                // Right side bar (auto placed in row 2, column 2)
735                (Node::DEFAULT, BackgroundColor(BLACK.into())),
736            ],
737        ));
738    }
```

Hide additional examples

examples/ui/layout/grid.rs ([line 40](../../src/grid/grid.rs.html#40))

```rust
18fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
19    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
20    commands.spawn(Camera2d);
21
22    // Top-level grid (app frame)
23    commands
24        .spawn((
25            Node {
26                // Use the CSS Grid algorithm for laying out this node
27                display: Display::Grid,
28                // Make node fill the entirety of its parent (in this case the window)
29                width: percent(100),
30                height: percent(100),
31                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
32                //   - The first column will size to the size of its contents
33                //   - The second column will take up the remaining available space
34                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
35                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
36                //  - The first row will size to the size of its contents
37                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
38                //  - The third row will be exactly 20px high
39                grid_template_rows: vec![
40                    GridTrack::auto(),
41                    GridTrack::flex(1.0),
42                    GridTrack::px(20.),
43                ],
44                ..default()
45            },
46            BackgroundColor(Color::WHITE),
47        ))
48        .with_children(|builder| {
49            // Header
50            builder
51                .spawn(
52                    Node {
53                        display: Display::Grid,
54                        // Make this node span two grid columns so that it takes up the entire top tow
55                        grid_column: GridPlacement::span(2),
56                        padding: UiRect::all(px(6)),
57                        ..default()
58                    },
59                )
60                .with_children(|builder| {
61                    spawn_nested_text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example");
62                });
63
64            // Main content grid (auto placed in row 2, column 1)
65            builder
66                .spawn((
67                    Node {
68                        // Make the height of the node fill its parent
69                        height: percent(100),
70                        // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
71                        // As the height is set explicitly, this means the width will adjust to match the height
72                        aspect_ratio: Some(1.0),
73                        // Use grid layout for this node
74                        display: Display::Grid,
75                        // Add 24px of padding around the grid
76                        padding: UiRect::all(px(24)),
77                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
78                        // This creates 4 exactly evenly sized columns
79                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
80                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
81                        // This creates 4 exactly evenly sized rows
82                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
83                        // Set a 12px gap/gutter between rows and columns
84                        row_gap: px(12),
85                        column_gap: px(12),
86                        ..default()
87                    },
88                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
89                ))
90                .with_children(|builder| {
91                    // Note there is no need to specify the position for each grid item. Grid items that are
92                    // not given an explicit position will be automatically positioned into the next available
93                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
94                    // style property.
95
96                    item_rect(builder, ORANGE);
97                    item_rect(builder, BISQUE);
98                    item_rect(builder, BLUE);
99                    item_rect(builder, CRIMSON);
100                    item_rect(builder, AQUA);
101                    item_rect(builder, ORANGE_RED);
102                    item_rect(builder, DARK_GREEN);
103                    item_rect(builder, FUCHSIA);
104                    item_rect(builder, TEAL);
105                    item_rect(builder, ALICE_BLUE);
106                    item_rect(builder, CRIMSON);
107                    item_rect(builder, ANTIQUE_WHITE);
108                    item_rect(builder, YELLOW);
109                    item_rect(builder, DEEP_PINK);
110                    item_rect(builder, YELLOW_GREEN);
111                    item_rect(builder, SALMON);
112                });
113
114            // Right side bar (auto placed in row 2, column 2)
115            builder
116                .spawn((
117                    Node {
118                        display: Display::Grid,
119                        // Align content towards the start (top) in the vertical axis
120                        align_items: AlignItems::Start,
121                        // Align content towards the center in the horizontal axis
122                        justify_items: JustifyItems::Center,
123                        // Add 10px padding
124                        padding: UiRect::all(px(10)),
125                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
126                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
127                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
128                        // Add a 10px gap between rows
129                        row_gap: px(10),
130                        ..default()
131                    },
132                    BackgroundColor(BLACK.into()),
133                ))
134                .with_children(|builder| {
135                    builder.spawn((Text::new("Sidebar"),
136                        TextFont::from(font.clone()),
137                    ));
138                    builder.spawn((Text::new("A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely."),
139                        TextFont {
140                            font: font.clone().into(),
141                            font_size: FontSize::Px(13.0),
142                            ..default()
143                        },
144                    ));
145                    builder.spawn(Node::default());
146                });
147
148            // Footer / status bar
149            builder.spawn((
150                Node {
151                    // Make this node span two grid column so that it takes up the entire bottom row
152                    grid_column: GridPlacement::span(2),
153                    ..default()
154                },
155                BackgroundColor(WHITE.into()),
156            ));
157
158            // Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
159            builder.spawn((
160                Node {
161                    position_type: PositionType::Absolute,
162                    margin: UiRect {
163                        top: px(100),
164                        bottom: auto(),
165                        left: auto(),
166                        right: auto(),
167                    },
168                    width: percent(60),
169                    height: px(300),
170                    max_width: px(600),
171                    ..default()
172                },
173                Visibility::Hidden,
174                BackgroundColor(Color::WHITE.with_alpha(0.8)),
175            ));
176        });
177}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1675)

#### pub fn [min\_content](#method.min_content)<T>() -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track which is automatically sized to fit its contents when sized at their “min-content” sizes

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 692](../../src/testbed_ui/ui.rs.html#692))

```rust
684    pub fn setup(mut commands: Commands) {
685        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Grid)));
686        // Top-level grid (app frame)
687        commands.spawn((
688            Node {
689                display: Display::Grid,
690                width: percent(100),
691                height: percent(100),
692                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
693                grid_template_rows: vec![
694                    GridTrack::auto(),
695                    GridTrack::flex(1.0),
696                    GridTrack::px(40.),
697                ],
698                ..default()
699            },
700            BackgroundColor(Color::WHITE),
701            DespawnOnExit(super::Scene::Grid),
702            children![
703                // Header
704                (
705                    Node {
706                        display: Display::Grid,
707                        grid_column: GridPlacement::span(2),
708                        padding: UiRect::all(px(40)),
709                        ..default()
710                    },
711                    BackgroundColor(RED.into()),
712                ),
713                // Main content grid (auto placed in row 2, column 1)
714                (
715                    Node {
716                        height: percent(100),
717                        aspect_ratio: Some(1.0),
718                        display: Display::Grid,
719                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
720                        grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
721                        row_gap: px(12),
722                        column_gap: px(12),
723                        ..default()
724                    },
725                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
726                    children![
727                        (Node::default(), BackgroundColor(ORANGE.into())),
728                        (Node::default(), BackgroundColor(BISQUE.into())),
729                        (Node::default(), BackgroundColor(BLUE.into())),
730                        (Node::default(), BackgroundColor(CRIMSON.into())),
731                        (Node::default(), BackgroundColor(AQUA.into())),
732                    ]
733                ),
734                // Right side bar (auto placed in row 2, column 2)
735                (Node::DEFAULT, BackgroundColor(BLACK.into())),
736            ],
737        ));
738    }
```

Hide additional examples

examples/ui/images/image\_node\_resizing.rs ([line 77](../../src/image_node_resizing/image_node_resizing.rs.html#77))

```rust
62fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
63    let image_handle = asset_server.load("branding/icon.png");
64    let full_text = format!(
65        "{}height : {}%, width : {}%",
66        TEXT_PREFIX, IMAGE_GROUP_BOX_INIT_HEIGHT, IMAGE_GROUP_BOX_INIT_WIDTH,
67    );
68
69    commands.spawn(Camera2d);
70
71    let container = commands
72        .spawn((
73            Node {
74                display: Display::Grid,
75                width: percent(100),
76                height: percent(100),
77                grid_template_rows: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
78                ..default()
79            },
80            BackgroundColor(Color::WHITE),
81        ))
82        .id();
83
84    // Keyboard Text
85    commands
86        .spawn((
87            TextData {
88                height: IMAGE_GROUP_BOX_INIT_HEIGHT,
89                width: IMAGE_GROUP_BOX_INIT_WIDTH,
90            },
91            Text::new(full_text),
92            TextColor::BLACK,
93            Node {
94                grid_row: GridPlacement::span(1),
95                padding: px(6).all(),
96                ..default()
97            },
98            UiDebugOptions {
99                enabled: false,
100                ..default()
101            },
102            ChildOf(container),
103        ))
104        .observe(update_text);
105
106    commands
107        .spawn((
108            Node {
109                display: Display::Flex,
110                grid_row: GridPlacement::span(1),
111                flex_direction: FlexDirection::Column,
112                justify_content: JustifyContent::SpaceAround,
113                padding: px(10.).all(),
114                ..default()
115            },
116            BackgroundColor(Color::BLACK),
117            ChildOf(container),
118        ))
119        .with_children(|builder| {
120            // `NodeImageMode::Auto` will resize the image automatically by taking the size of the source image and applying any layout constraints.
121            builder
122                .spawn((
123                    ImageGroup,
124                    Node {
125                        display: Display::Flex,
126                        justify_content: JustifyContent::Start,
127                        width: percent(IMAGE_GROUP_BOX_INIT_WIDTH),
128                        height: percent(IMAGE_GROUP_BOX_INIT_HEIGHT),
129                        ..default()
130                    },
131                    BackgroundColor(Color::from(tailwind::BLUE_100)),
132                ))
133                .with_children(|parent| {
134                    for _ in 0..4 {
135                        // child node will apply Flex layout
136                        parent.spawn((
137                            Node::default(),
138                            ImageNode {
139                                image: image_handle.clone(),
140                                image_mode: NodeImageMode::Auto,
141                                ..default()
142                            },
143                        ));
144                    }
145                });
146            // `NodeImageMode::Stretch` will resize the image to match the size of the `Node` component
147            builder
148                .spawn((
149                    ImageGroup,
150                    Node {
151                        display: Display::Flex,
152                        justify_content: JustifyContent::Start,
153                        width: percent(IMAGE_GROUP_BOX_INIT_WIDTH),
154                        height: percent(IMAGE_GROUP_BOX_INIT_HEIGHT),
155                        ..default()
156                    },
157                    BackgroundColor(Color::from(tailwind::BLUE_100)),
158                ))
159                .with_children(|parent| {
160                    for width in [10., 20., 30., 40.] {
161                        parent.spawn((
162                            Node {
163                                height: percent(100),
164                                width: percent(width),
165                                ..default()
166                            },
167                            ImageNode {
168                                image: image_handle.clone(),
169                                image_mode: NodeImageMode::Stretch,
170                                ..default()
171                            },
172                        ));
173                    }
174                });
175        });
176}
```

examples/ui/layout/grid.rs ([line 34](../../src/grid/grid.rs.html#34))

```rust
18fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
19    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
20    commands.spawn(Camera2d);
21
22    // Top-level grid (app frame)
23    commands
24        .spawn((
25            Node {
26                // Use the CSS Grid algorithm for laying out this node
27                display: Display::Grid,
28                // Make node fill the entirety of its parent (in this case the window)
29                width: percent(100),
30                height: percent(100),
31                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
32                //   - The first column will size to the size of its contents
33                //   - The second column will take up the remaining available space
34                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
35                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
36                //  - The first row will size to the size of its contents
37                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
38                //  - The third row will be exactly 20px high
39                grid_template_rows: vec![
40                    GridTrack::auto(),
41                    GridTrack::flex(1.0),
42                    GridTrack::px(20.),
43                ],
44                ..default()
45            },
46            BackgroundColor(Color::WHITE),
47        ))
48        .with_children(|builder| {
49            // Header
50            builder
51                .spawn(
52                    Node {
53                        display: Display::Grid,
54                        // Make this node span two grid columns so that it takes up the entire top tow
55                        grid_column: GridPlacement::span(2),
56                        padding: UiRect::all(px(6)),
57                        ..default()
58                    },
59                )
60                .with_children(|builder| {
61                    spawn_nested_text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example");
62                });
63
64            // Main content grid (auto placed in row 2, column 1)
65            builder
66                .spawn((
67                    Node {
68                        // Make the height of the node fill its parent
69                        height: percent(100),
70                        // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
71                        // As the height is set explicitly, this means the width will adjust to match the height
72                        aspect_ratio: Some(1.0),
73                        // Use grid layout for this node
74                        display: Display::Grid,
75                        // Add 24px of padding around the grid
76                        padding: UiRect::all(px(24)),
77                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
78                        // This creates 4 exactly evenly sized columns
79                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
80                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
81                        // This creates 4 exactly evenly sized rows
82                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
83                        // Set a 12px gap/gutter between rows and columns
84                        row_gap: px(12),
85                        column_gap: px(12),
86                        ..default()
87                    },
88                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
89                ))
90                .with_children(|builder| {
91                    // Note there is no need to specify the position for each grid item. Grid items that are
92                    // not given an explicit position will be automatically positioned into the next available
93                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
94                    // style property.
95
96                    item_rect(builder, ORANGE);
97                    item_rect(builder, BISQUE);
98                    item_rect(builder, BLUE);
99                    item_rect(builder, CRIMSON);
100                    item_rect(builder, AQUA);
101                    item_rect(builder, ORANGE_RED);
102                    item_rect(builder, DARK_GREEN);
103                    item_rect(builder, FUCHSIA);
104                    item_rect(builder, TEAL);
105                    item_rect(builder, ALICE_BLUE);
106                    item_rect(builder, CRIMSON);
107                    item_rect(builder, ANTIQUE_WHITE);
108                    item_rect(builder, YELLOW);
109                    item_rect(builder, DEEP_PINK);
110                    item_rect(builder, YELLOW_GREEN);
111                    item_rect(builder, SALMON);
112                });
113
114            // Right side bar (auto placed in row 2, column 2)
115            builder
116                .spawn((
117                    Node {
118                        display: Display::Grid,
119                        // Align content towards the start (top) in the vertical axis
120                        align_items: AlignItems::Start,
121                        // Align content towards the center in the horizontal axis
122                        justify_items: JustifyItems::Center,
123                        // Add 10px padding
124                        padding: UiRect::all(px(10)),
125                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
126                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
127                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
128                        // Add a 10px gap between rows
129                        row_gap: px(10),
130                        ..default()
131                    },
132                    BackgroundColor(BLACK.into()),
133                ))
134                .with_children(|builder| {
135                    builder.spawn((Text::new("Sidebar"),
136                        TextFont::from(font.clone()),
137                    ));
138                    builder.spawn((Text::new("A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely."),
139                        TextFont {
140                            font: font.clone().into(),
141                            font_size: FontSize::Px(13.0),
142                            ..default()
143                        },
144                    ));
145                    builder.spawn(Node::default());
146                });
147
148            // Footer / status bar
149            builder.spawn((
150                Node {
151                    // Make this node span two grid column so that it takes up the entire bottom row
152                    grid_column: GridPlacement::span(2),
153                    ..default()
154                },
155                BackgroundColor(WHITE.into()),
156            ));
157
158            // Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
159            builder.spawn((
160                Node {
161                    position_type: PositionType::Absolute,
162                    margin: UiRect {
163                        top: px(100),
164                        bottom: auto(),
165                        left: auto(),
166                        right: auto(),
167                    },
168                    width: percent(60),
169                    height: px(300),
170                    max_width: px(600),
171                    ..default()
172                },
173                Visibility::Hidden,
174                BackgroundColor(Color::WHITE.with_alpha(0.8)),
175            ));
176        });
177}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1684)

#### pub fn [max\_content](#method.max_content)<T>() -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track which is automatically sized to fit its contents when sized at their “max-content” sizes

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1695)

#### pub fn [fit\_content\_px](#method.fit_content_px)<T>(limit: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a `fit-content()` grid track with fixed pixel limit.

[https://developer.mozilla.org/en-US/docs/Web/CSS/fit-content\_function](https://developer.mozilla.org/en-US/docs/Web/CSS/fit-content_function)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1706)

#### pub fn [fit\_content\_percent](#method.fit_content_percent)<T>(limit: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a `fit-content()` grid track with percentage limit.

[https://developer.mozilla.org/en-US/docs/Web/CSS/fit-content\_function](https://developer.mozilla.org/en-US/docs/Web/CSS/fit-content_function)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1717)

#### pub fn [minmax](#method.minmax)<T>(min: [MinTrackSizingFunction](enum.MinTrackSizingFunction.html "enum bevy::prelude::MinTrackSizingFunction"), max: [MaxTrackSizingFunction](enum.MaxTrackSizingFunction.html "enum bevy::prelude::MaxTrackSizingFunction")) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a `minmax()` grid track.

[https://developer.mozilla.org/en-US/docs/Web/CSS/minmax](https://developer.mozilla.org/en-US/docs/Web/CSS/minmax)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1726)

#### pub fn [vmin](#method.vmin)<T>(value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track with a percentage of the viewport’s smaller dimension

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1735)

#### pub fn [vmax](#method.vmax)<T>(value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track with a percentage of the viewport’s larger dimension

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1744)

#### pub fn [vh](#method.vh)<T>(value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track with a percentage of the viewport’s height dimension

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1753)

#### pub fn [vw](#method.vw)<T>(value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>,

Create a grid track with a percentage of the viewport’s width dimension

## Trait Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1762)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1763)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1613)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1613)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1990)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\> for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1991)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(track: [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")) -> [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1999)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\> for [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2000)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(track: [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")) -> [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2005)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\> for [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2006)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(track: [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")) -> [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [from\_reflect](trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [get\_represented\_type\_info](trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [try\_apply](trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [reflect\_kind](trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [reflect\_ref](trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [reflect\_mut](trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [reflect\_owned](trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [try\_into\_reflect](trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](struct.Box.html "struct bevy::prelude::Box")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [try\_as\_reflect](trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [try\_as\_reflect\_mut](trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [into\_partial\_reflect](trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [as\_partial\_reflect](trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [as\_partial\_reflect\_mut](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1610)

#### fn [reflect\_partial\_eq](trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [reflect\_partial\_cmp](trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1610)

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [into\_any](trait.Reflect.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [as\_any](trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [as\_any\_mut](trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [into\_reflect](trait.Reflect.html#tymethod.into_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [as\_reflect](trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [as\_reflect\_mut](trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [set](trait.Reflect.html#tymethod.set)(&mut self, value: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1613)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1613)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [field](trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [field\_mut](trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [field\_at](trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [field\_at\_mut](trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [name\_at](trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [index\_of\_name](trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [field\_len](trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [iter\_fields](trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [to\_dynamic\_struct](trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [type\_path](trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [short\_type\_path](trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [type\_ident](trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [crate\_name](trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [module\_path](trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

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