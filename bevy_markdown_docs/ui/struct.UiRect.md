[bevy](../index.html)::[ui](index.html)

# Struct UiRect 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#632)

```rust
pub struct UiRect {
    pub left: Val,
    pub right: Val,
    pub top: Val,
    pub bottom: Val,
}
```

A type which is commonly used to define margins, paddings and borders.

## Examples

### Margin

A margin is used to create space around UI elements, outside of any defined borders.

```rust
let margin = UiRect::all(Val::Auto); // Centers the UI element
```

### Padding

A padding is used to create space around UI elements, inside of any defined borders.

```rust
let padding = UiRect {
    left: Val::Px(10.0),
    right: Val::Px(20.0),
    top: Val::Px(30.0),
    bottom: Val::Px(40.0),
};
```

### Borders

A border is used to define the width of the border of a UI element.

```rust
let border = UiRect {
    left: Val::Px(10.0),
    right: Val::Px(20.0),
    top: Val::Px(30.0),
    bottom: Val::Px(40.0),
};
```

## Fields

`left: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")`

The value corresponding to the left side of the UI rect.

`right: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")`

The value corresponding to the right side of the UI rect.

`top: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")`

The value corresponding to the top side of the UI rect.

`bottom: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")`

The value corresponding to the bottom side of the UI rect.

## Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#643)

### impl [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#644)

#### pub const [DEFAULT](#associatedconstant.DEFAULT): [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#645)

#### pub const [ZERO](#associatedconstant.ZERO): [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#646)

#### pub const [AUTO](#associatedconstant.AUTO): [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#667)

#### pub const fn [new](#method.new)(left: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val"), right: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val"), top: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val"), bottom: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") from the values specified.

##### Example

```rust
let ui_rect = UiRect::new(
    Val::Px(10.0),
    Val::Px(20.0),
    Val::Px(30.0),
    Val::Px(40.0),
);

assert_eq!(ui_rect.left, Val::Px(10.0));
assert_eq!(ui_rect.right, Val::Px(20.0));
assert_eq!(ui_rect.top, Val::Px(30.0));
assert_eq!(ui_rect.bottom, Val::Px(40.0));
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#690)

#### pub const fn [all](#method.all)(value: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") where all sides have the same value.

##### Example

```rust
let ui_rect = UiRect::all(Val::Px(10.0));

assert_eq!(ui_rect.left, Val::Px(10.0));
assert_eq!(ui_rect.right, Val::Px(10.0));
assert_eq!(ui_rect.top, Val::Px(10.0));
assert_eq!(ui_rect.bottom, Val::Px(10.0));
```

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/3d/../helpers/widgets.rs ([line 28](../../src/clustered_decal_maps/helpers/widgets.rs.html#28))

```rust
28pub const BUTTON_BORDER: UiRect = UiRect::all(Val::Px(1.0));
```

Hide additional examples

examples/app/log\_layers\_ecs.rs ([line 135](../../src/log_layers_ecs/log_layers_ecs.rs.html#135))

```rust
127fn setup(mut commands: Commands) {
128    commands.spawn(Camera2d);
129
130    commands.spawn((
131        Node {
132            width: vw(100),
133            height: vh(100),
134            flex_direction: FlexDirection::Column,
135            padding: UiRect::all(px(12)),
136            ..default()
137        },
138        LogViewerRoot,
139    ));
140}
```

examples/usage/context\_menu.rs ([line 131](../../src/context_menu/context_menu.rs.html#131))

```rust
125fn context_item(text: &str, col: Srgba) -> impl Bundle {
126    (
127        Name::new(format!("item-{text}")),
128        ContextMenuItem(col),
129        Button,
130        Node {
131            padding: UiRect::all(px(5)),
132            ..default()
133        },
134        children![(
135            Pickable::IGNORE,
136            Text::new(text),
137            TextFont {
138                font_size: FontSize::Px(24.0),
139                ..default()
140            },
141            TextColor(Color::WHITE),
142        )],
143    )
144}
145
146fn background_and_button() -> impl Bundle {
147    (
148        Name::new("background"),
149        Node {
150            width: percent(100),
151            height: percent(100),
152            align_items: AlignItems::Center,
153            justify_content: JustifyContent::Center,
154            ..default()
155        },
156        ZIndex(-10),
157        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
158            parent
159                .spawn((
160                    Name::new("button"),
161                    Button,
162                    Node {
163                        width: px(250),
164                        height: px(65),
165                        border: UiRect::all(px(5)),
166                        justify_content: JustifyContent::Center,
167                        align_items: AlignItems::Center,
168                        border_radius: BorderRadius::MAX,
169                        ..default()
170                    },
171                    BorderColor::all(Color::BLACK),
172                    BackgroundColor(Color::BLACK),
173                    children![(
174                        Pickable::IGNORE,
175                        Text::new("Context Menu"),
176                        TextFont {
177                            font_size: FontSize::Px(28.0),
178                            ..default()
179                        },
180                        TextColor(Color::WHITE),
181                        TextShadow::default(),
182                    )],
183                ))
184                .observe(|mut event: On<Pointer<Press>>, mut commands: Commands| {
185                    // by default this event would bubble up further leading to the `CloseContextMenus`
186                    // event being triggered and undoing the opening of one here right away.
187                    event.propagate(false);
188
189                    debug!("click: {}", event.pointer_location.position);
190
191                    commands.trigger(OpenContextMenu {
192                        pos: event.pointer_location.position,
193                    });
194                });
195        })),
196    )
197}
```

examples/ui/layout/ghost\_nodes.rs ([line 78](../../src/ghost_nodes/ghost_nodes.rs.html#78))

```rust
72fn create_button() -> impl Bundle {
73    (
74        Button,
75        Node {
76            width: px(150),
77            height: px(65),
78            border: UiRect::all(px(5)),
79            // horizontally center child text
80            justify_content: JustifyContent::Center,
81            // vertically center child text
82            align_items: AlignItems::Center,
83            border_radius: BorderRadius::MAX,
84            ..default()
85        },
86        BorderColor::all(Color::BLACK),
87        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
88    )
89}
```

examples/3d/split\_screen.rs ([line 117](../../src/split_screen/split_screen.rs.html#117))

```rust
107    fn buttons_panel() -> impl Bundle {
108        (
109            Node {
110                position_type: PositionType::Absolute,
111                width: percent(100),
112                height: percent(100),
113                display: Display::Flex,
114                flex_direction: FlexDirection::Row,
115                justify_content: JustifyContent::SpaceBetween,
116                align_items: AlignItems::Center,
117                padding: UiRect::all(px(20)),
118                ..default()
119            },
120            children![
121                rotate_button("<", Direction::Left),
122                rotate_button(">", Direction::Right),
123            ],
124        )
125    }
126
127    fn rotate_button(caption: &str, direction: Direction) -> impl Bundle {
128        (
129            RotateCamera(direction),
130            Button,
131            Node {
132                width: px(40),
133                height: px(40),
134                border: UiRect::all(px(2)),
135                justify_content: JustifyContent::Center,
136                align_items: AlignItems::Center,
137                ..default()
138            },
139            BorderColor::all(Color::WHITE),
140            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
141            children![Text::new(caption)],
142        )
143    }
```

examples/ui/scroll\_and\_overflow/scrollbars.rs ([line 32](../../src/scrollbars/scrollbars.rs.html#32))

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
42
43/// Create a scrolling area.
44///
45/// The "scroll area" is a container that can be scrolled. It has a nested structure which is
46/// three levels deep:
47/// - The outermost node is a grid that contains the scroll area and the scrollbars.
48/// - The scroll area is a flex container that contains the scrollable content. This
49///   is the element that has the `overflow: scroll` property.
50/// - The scrollable content consists of the elements actually displayed in the scrolling area.
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

Additional examples can be found in:  

*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#197)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#128)
*   [examples/window/scale\_factor\_override.rs](../../src/scale_factor_override/scale_factor_override.rs.html#41)
*   [examples/window/window\_drag\_move.rs](../../src/window_drag_move/window_drag_move.rs.html#66)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#188)
*   [examples/ui/widgets/button.rs](../../src/button/button.rs.html#91)
*   [examples/ui/ui\_material.rs](../../src/ui_material/ui_material.rs.html#43)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#59)
*   [examples/remote/app\_under\_test.rs](../../src/app_under_test/app_under_test.rs.html#88)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#242)
*   [examples/stress\_tests/many\_buttons.rs](../../src/many_buttons/many_buttons.rs.html#166)
*   [examples/ui/images/ui\_texture\_slice.rs](../../src/ui_texture_slice/ui_texture_slice.rs.html#80)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#81)
*   [examples/camera/2d\_on\_ui.rs](../../src/2d_on_ui/2d_on_ui.rs.html#47)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#86)
*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#708)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#90)
*   [examples/picking/dragdrop\_picking.rs](../../src/dragdrop_picking/dragdrop_picking.rs.html#54)
*   [examples/ui/layout/anchor\_layout.rs](../../src/anchor_layout/anchor_layout.rs.html#59)
*   [examples/showcase/stepping.rs](../../src/breakout/stepping.rs.html#177)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#171)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../src/overflow/overflow.rs.html#50)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#47)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#236)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#263)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#284)
*   [examples/3d/shadow\_biases.rs](../../src/shadow_biases/shadow_biases.rs.html#101)
*   [examples/ui/widgets/tab\_navigation.rs](../../src/tab_navigation/tab_navigation.rs.html#122)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../src/scroll/scroll.rs.html#151)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#98)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#171)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#141)
*   [examples/ui/styling/box\_shadow.rs](../../src/box_shadow/box_shadow.rs.html#162)
*   [examples/ui/text/text.rs](../../src/text/text.rs.html#90)
*   [examples/ui/styling/borders.rs](../../src/borders/borders.rs.html#39)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#191)
*   [examples/ui/layout/grid.rs](../../src/grid/grid.rs.html#56)
*   [examples/ui/ui\_transform.rs](../../src/ui_transform/ui_transform.rs.html#132)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#34)
*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#50)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#714)

#### pub const fn [px](#method.px)(left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), top: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") from the values specified in logical pixels.

This is a shortcut for [`UiRect::new()`](../prelude/struct.UiRect.html#method.new "associated function bevy::prelude::UiRect::new"), applying [`Val::Px`](../prelude/enum.Val.html#variant.Px "variant bevy::prelude::Val::Px") to all arguments.

##### Example

```rust
let ui_rect = UiRect::px(10., 20., 30., 40.);
assert_eq!(ui_rect.left, Val::Px(10.));
assert_eq!(ui_rect.right, Val::Px(20.));
assert_eq!(ui_rect.top, Val::Px(30.));
assert_eq!(ui_rect.bottom, Val::Px(40.));
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#738)

#### pub const fn [percent](#method.percent)(left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), top: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") from the values specified in percentages.

This is a shortcut for [`UiRect::new()`](../prelude/struct.UiRect.html#method.new "associated function bevy::prelude::UiRect::new"), applying [`Val::Percent`](../prelude/enum.Val.html#variant.Percent "variant bevy::prelude::Val::Percent") to all arguments.

##### Example

```rust
let ui_rect = UiRect::percent(5., 10., 2., 1.);
assert_eq!(ui_rect.left, Val::Percent(5.));
assert_eq!(ui_rect.right, Val::Percent(10.));
assert_eq!(ui_rect.top, Val::Percent(2.));
assert_eq!(ui_rect.bottom, Val::Percent(1.));
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#762)

#### pub const fn [horizontal](#method.horizontal)(value: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") where `left` and `right` take the given value, and `top` and `bottom` set to zero `Val::ZERO`.

##### Example

```rust
let ui_rect = UiRect::horizontal(Val::Px(10.0));

assert_eq!(ui_rect.left, Val::Px(10.0));
assert_eq!(ui_rect.right, Val::Px(10.0));
assert_eq!(ui_rect.top, Val::ZERO);
assert_eq!(ui_rect.bottom, Val::ZERO);
```

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/3d/ssr.rs ([line 546](../../src/ssr/ssr.rs.html#546))

```rust
513fn range_row(
514    title: &str,
515    start_value: f32,
516    end_value: f32,
517    start_marker: RangeValueText,
518    end_marker: RangeValueText,
519    start_dec: ExampleSetting,
520    start_inc: ExampleSetting,
521    end_dec: ExampleSetting,
522    end_inc: ExampleSetting,
523) -> impl Bundle {
524    (
525        Node {
526            align_items: AlignItems::Center,
527            ..default()
528        },
529        Children::spawn((
530            Spawn((
531                widgets::ui_text(title, Color::WHITE),
532                Node {
533                    width: px(150),
534                    ..default()
535                },
536            )),
537            Spawn(range_controls(
538                start_value,
539                start_marker,
540                start_dec,
541                start_inc,
542            )),
543            Spawn((
544                widgets::ui_text("to", Color::WHITE),
545                Node {
546                    margin: UiRect::horizontal(px(10)),
547                    ..default()
548                },
549            )),
550            Spawn(range_controls(end_value, end_marker, end_dec, end_inc)),
551        )),
552    )
553}
```

Hide additional examples

examples/ui/layout/size\_constraints.rs ([line 221](../../src/size_constraints/size_constraints.rs.html#221))

```rust
206fn spawn_button(
207    parent: &mut ChildSpawnerCommands,
208    constraint: Constraint,
209    action: ButtonValue,
210    label: String,
211    text_style: (TextFont, TextColor),
212    active: bool,
213) {
214    parent
215        .spawn((
216            Button,
217            Node {
218                align_items: AlignItems::Center,
219                justify_content: JustifyContent::Center,
220                border: UiRect::all(px(2)),
221                margin: UiRect::horizontal(px(2)),
222                ..Default::default()
223            },
224            BorderColor::all(if active {
225                ACTIVE_BORDER_COLOR
226            } else {
227                INACTIVE_BORDER_COLOR
228            }),
229            constraint,
230            action,
231        ))
232        .with_children(|parent| {
233            parent
234                .spawn((
235                    Node {
236                        width: px(100),
237                        justify_content: JustifyContent::Center,
238                        ..default()
239                    },
240                    BackgroundColor(if active {
241                        ACTIVE_INNER_COLOR
242                    } else {
243                        INACTIVE_INNER_COLOR
244                    }),
245                ))
246                .with_child((
247                    Text::new(label),
248                    text_style.0,
249                    TextColor(if active {
250                        ACTIVE_TEXT_COLOR
251                    } else {
252                        UNHOVERED_TEXT_COLOR
253                    }),
254                    TextLayout::justify(Justify::Center),
255                ));
256        });
257}
```

examples/ui/layout/anchor\_layout.rs ([line 82](../../src/anchor_layout/anchor_layout.rs.html#82))

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

examples/ui/scroll\_and\_overflow/overflow.rs ([line 42](../../src/overflow/overflow.rs.html#42))

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

examples/ui/styling/box\_shadow.rs ([line 358](../../src/box_shadow/box_shadow.rs.html#358))

```rust
290fn build_setting_row(
291    setting_type: SettingType,
292    dec: SettingsButton,
293    inc: SettingsButton,
294    value: f32,
295    asset_server: &Res<AssetServer>,
296) -> impl Bundle {
297    let value_text = match setting_type {
298        SettingType::Shape => SHAPES[value as usize % SHAPES.len()].0.to_string(),
299        SettingType::Count => format!("{}", value as usize),
300        _ => format!("{value:.1}"),
301    };
302
303    (
304        Node {
305            flex_direction: FlexDirection::Row,
306            align_items: AlignItems::Center,
307            height: px(32),
308            ..default()
309        },
310        children![
311            (
312                Node {
313                    width: px(80),
314                    justify_content: JustifyContent::FlexEnd,
315                    align_items: AlignItems::Center,
316                    ..default()
317                },
318                // Attach SettingType to the value label node, not the parent row
319                children![(
320                    Text::new(setting_type.label()),
321                    TextFont {
322                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
323                        font_size: FontSize::Px(16.0),
324                        ..default()
325                    },
326                )],
327            ),
328            (
329                Button,
330                Node {
331                    width: px(28),
332                    height: px(28),
333                    margin: UiRect::left(px(8)),
334                    justify_content: JustifyContent::Center,
335                    align_items: AlignItems::Center,
336                    border_radius: BorderRadius::all(px(6)),
337                    ..default()
338                },
339                BackgroundColor(Color::WHITE),
340                dec,
341                children![(
342                    Text::new(if setting_type == SettingType::Shape {
343                        "<"
344                    } else {
345                        "-"
346                    }),
347                    TextFont {
348                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
349                        font_size: FontSize::Px(18.0),
350                        ..default()
351                    },
352                )],
353            ),
354            (
355                Node {
356                    width: px(48),
357                    height: px(28),
358                    margin: UiRect::horizontal(px(8)),
359                    justify_content: JustifyContent::Center,
360                    align_items: AlignItems::Center,
361                    border_radius: BorderRadius::all(px(6)),
362                    ..default()
363                },
364                children![{
365                    (
366                        Text::new(value_text),
367                        TextFont {
368                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
369                            font_size: FontSize::Px(16.0),
370                            ..default()
371                        },
372                        setting_type,
373                    )
374                }],
375            ),
376            (
377                Button,
378                Node {
379                    width: px(28),
380                    height: px(28),
381                    justify_content: JustifyContent::Center,
382                    align_items: AlignItems::Center,
383                    border_radius: BorderRadius::all(px(6)),
384                    ..default()
385                },
386                BackgroundColor(Color::WHITE),
387                inc,
388                children![(
389                    Text::new(if setting_type == SettingType::Shape {
390                        ">"
391                    } else {
392                        "+"
393                    }),
394                    TextFont {
395                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
396                        font_size: FontSize::Px(18.0),
397                        ..default()
398                    },
399                )],
400            ),
401        ],
402    )
403}
```

examples/ui/styling/borders.rs ([line 44](../../src/borders/borders.rs.html#44))

```rust
12fn setup(mut commands: Commands) {
13    commands.spawn(Camera2d);
14
15    // labels for the different border edges
16    let border_labels = [
17        "None",
18        "All",
19        "Left",
20        "Right",
21        "Top",
22        "Bottom",
23        "Horizontal",
24        "Vertical",
25        "Top Left",
26        "Bottom Left",
27        "Top Right",
28        "Bottom Right",
29        "Top Bottom Right",
30        "Top Bottom Left",
31        "Top Left Right",
32        "Bottom Left Right",
33    ];
34
35    // all the different combinations of border edges
36    // these correspond to the labels above
37    let borders = [
38        UiRect::default(),
39        UiRect::all(px(10)),
40        UiRect::left(px(10)),
41        UiRect::right(px(10)),
42        UiRect::top(px(10)),
43        UiRect::bottom(px(10)),
44        UiRect::horizontal(px(10)),
45        UiRect::vertical(px(10)),
46        UiRect {
47            left: px(20),
48            top: px(10),
49            ..default()
50        },
51        UiRect {
52            left: px(10),
53            bottom: px(20),
54            ..default()
55        },
56        UiRect {
57            right: px(20),
58            top: px(10),
59            ..default()
60        },
61        UiRect {
62            right: px(10),
63            bottom: px(10),
64            ..default()
65        },
66        UiRect {
67            right: px(10),
68            top: px(20),
69            bottom: px(10),
70            ..default()
71        },
72        UiRect {
73            left: px(10),
74            top: px(10),
75            bottom: px(10),
76            ..default()
77        },
78        UiRect {
79            left: px(20),
80            right: px(10),
81            top: px(10),
82            ..default()
83        },
84        UiRect {
85            left: px(10),
86            right: px(10),
87            bottom: px(20),
88            ..default()
89        },
90    ];
91
92    let borders_examples = (
93        Node {
94            margin: px(25).all(),
95            flex_wrap: FlexWrap::Wrap,
96            ..default()
97        },
98        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
99            |(label, border)| {
100                (
101                    Node {
102                        flex_direction: FlexDirection::Column,
103                        align_items: AlignItems::Center,
104                        ..default()
105                    },
106                    children![
107                        (
108                            Node {
109                                width: px(50),
110                                height: px(50),
111                                border,
112                                margin: px(20).all(),
113                                align_items: AlignItems::Center,
114                                justify_content: JustifyContent::Center,
115                                ..default()
116                            },
117                            BackgroundColor(MAROON.into()),
118                            BorderColor {
119                                top: RED.into(),
120                                bottom: YELLOW.into(),
121                                left: GREEN.into(),
122                                right: BLUE.into(),
123                            },
124                            Outline {
125                                width: px(6),
126                                offset: px(6),
127                                color: Color::WHITE,
128                            },
129                            children![(
130                                Node {
131                                    width: px(10),
132                                    height: px(10),
133                                    ..default()
134                                },
135                                BackgroundColor(YELLOW.into()),
136                            )]
137                        ),
138                        (Text::new(label), TextFont::from_font_size(9.0))
139                    ],
140                )
141            },
142        ))),
143    );
144
145    let non_zero = |x, y| x != px(0) && y != px(0);
146    let border_size = move |x, y| {
147        if non_zero(x, y) {
148            f32::MAX
149        } else {
150            0.
151        }
152    };
153
154    let borders_examples_rounded = (
155        Node {
156            margin: px(25).all(),
157            flex_wrap: FlexWrap::Wrap,
158            ..default()
159        },
160        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
161            move |(label, border)| {
162                (
163                    Node {
164                        flex_direction: FlexDirection::Column,
165                        align_items: AlignItems::Center,
166                        ..default()
167                    },
168                    children![
169                        (
170                            Node {
171                                width: px(50),
172                                height: px(50),
173                                border,
174                                margin: px(20).all(),
175                                align_items: AlignItems::Center,
176                                justify_content: JustifyContent::Center,
177                                border_radius: BorderRadius::px(
178                                    border_size(border.left, border.top),
179                                    border_size(border.right, border.top),
180                                    border_size(border.right, border.bottom,),
181                                    border_size(border.left, border.bottom),
182                                ),
183                                ..default()
184                            },
185                            BackgroundColor(MAROON.into()),
186                            BorderColor {
187                                top: RED.into(),
188                                bottom: YELLOW.into(),
189                                left: GREEN.into(),
190                                right: BLUE.into(),
191                            },
192                            Outline {
193                                width: px(6),
194                                offset: px(6),
195                                color: Color::WHITE,
196                            },
197                            children![(
198                                Node {
199                                    width: px(10),
200                                    height: px(10),
201                                    border_radius: BorderRadius::MAX,
202                                    ..default()
203                                },
204                                BackgroundColor(YELLOW.into()),
205                            )],
206                        ),
207                        (Text::new(label), TextFont::from_font_size(9.0))
208                    ],
209                )
210            },
211        ))),
212    );
213
214    commands.spawn((
215        Node {
216            margin: px(25).all(),
217            flex_direction: FlexDirection::Column,
218            align_self: AlignSelf::Stretch,
219            justify_self: JustifySelf::Stretch,
220            ..default()
221        },
222        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
223        children![
224            label("Borders"),
225            borders_examples,
226            label("Borders Rounded"),
227            borders_examples_rounded
228        ],
229    ));
230}
```

Additional examples can be found in:  

*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#233)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#785)

#### pub const fn [vertical](#method.vertical)(value: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") where `top` and `bottom` take the given value, and `left` and `right` are set to `Val::ZERO`.

##### Example

```rust
let ui_rect = UiRect::vertical(Val::Px(10.0));

assert_eq!(ui_rect.left, Val::ZERO);
assert_eq!(ui_rect.right, Val::ZERO);
assert_eq!(ui_rect.top, Val::Px(10.0));
assert_eq!(ui_rect.bottom, Val::Px(10.0));
```

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/ui/layout/anchor\_layout.rs ([line 52](../../src/anchor_layout/anchor_layout.rs.html#52))

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

Hide additional examples

examples/testbed/ui.rs ([line 761](../../src/testbed_ui/ui.rs.html#761))

```rust
744    pub fn setup(mut commands: Commands) {
745        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Borders)));
746        let root = commands
747            .spawn((
748                Node {
749                    flex_wrap: FlexWrap::Wrap,
750                    ..default()
751                },
752                DespawnOnExit(super::Scene::Borders),
753            ))
754            .id();
755
756        // all the different combinations of border edges
757        let borders = [
758            UiRect::default(),
759            UiRect::all(px(20)),
760            UiRect::left(px(20)),
761            UiRect::vertical(px(20)),
762            UiRect {
763                left: px(40),
764                top: px(20),
765                ..Default::default()
766            },
767            UiRect {
768                right: px(20),
769                bottom: px(30),
770                ..Default::default()
771            },
772            UiRect {
773                right: px(20),
774                top: px(40),
775                bottom: px(20),
776                ..Default::default()
777            },
778            UiRect {
779                left: px(20),
780                top: px(20),
781                bottom: px(20),
782                ..Default::default()
783            },
784            UiRect {
785                left: px(20),
786                right: px(20),
787                bottom: px(40),
788                ..Default::default()
789            },
790        ];
791
792        let non_zero = |x, y| x != px(0) && y != px(0);
793        let border_size = |x, y| if non_zero(x, y) { f32::MAX } else { 0. };
794
795        for border in borders {
796            for rounded in [true, false] {
797                let border_node = commands
798                    .spawn((
799                        Node {
800                            width: px(100),
801                            height: px(100),
802                            border,
803                            margin: UiRect::all(px(30)),
804                            align_items: AlignItems::Center,
805                            justify_content: JustifyContent::Center,
806                            border_radius: if rounded {
807                                BorderRadius::px(
808                                    border_size(border.left, border.top),
809                                    border_size(border.right, border.top),
810                                    border_size(border.right, border.bottom),
811                                    border_size(border.left, border.bottom),
812                                )
813                            } else {
814                                BorderRadius::ZERO
815                            },
816                            ..default()
817                        },
818                        BackgroundColor(MAROON.into()),
819                        BorderColor::all(RED),
820                        Outline {
821                            width: px(10),
822                            offset: px(10),
823                            color: Color::WHITE,
824                        },
825                    ))
826                    .id();
827
828                commands.entity(root).add_child(border_node);
829            }
830        }
831    }
```

examples/animation/animation\_masks.rs ([line 274](../../src/animation_masks/animation_masks.rs.html#274))

```rust
229fn new_mask_group_control(label: &str, width: Val, mask_group_id: u32) -> impl Bundle {
230    let button_text_style = (
231        TextFont {
232            font_size: FontSize::Px(14.0),
233            ..default()
234        },
235        TextColor::WHITE,
236    );
237    let selected_button_text_style = (button_text_style.0.clone(), TextColor::BLACK);
238    let label_text_style = (
239        button_text_style.0.clone(),
240        TextColor(Color::Srgba(LIGHT_GRAY)),
241    );
242
243    let make_animation_label = {
244        let button_text_style = button_text_style.clone();
245        let selected_button_text_style = selected_button_text_style.clone();
246        move |first: bool, label: AnimationLabel| {
247            (
248                Button,
249                BackgroundColor(if !first { Color::BLACK } else { Color::WHITE }),
250                Node {
251                    flex_grow: 1.0,
252                    border: if !first {
253                        UiRect::left(px(1))
254                    } else {
255                        UiRect::ZERO
256                    },
257                    ..default()
258                },
259                BorderColor::all(Color::WHITE),
260                AnimationControl {
261                    group_id: mask_group_id,
262                    label,
263                },
264                children![(
265                    Text(format!("{label:?}")),
266                    if !first {
267                        button_text_style.clone()
268                    } else {
269                        selected_button_text_style.clone()
270                    },
271                    TextLayout::justify(Justify::Center),
272                    Node {
273                        flex_grow: 1.0,
274                        margin: UiRect::vertical(px(3)),
275                        ..default()
276                    },
277                )],
278            )
279        }
280    };
281
282    (
283        Node {
284            border: UiRect::all(px(1)),
285            width,
286            flex_direction: FlexDirection::Column,
287            justify_content: JustifyContent::Center,
288            align_items: AlignItems::Center,
289            padding: UiRect::ZERO,
290            margin: UiRect::ZERO,
291            border_radius: BorderRadius::all(px(3)),
292            ..default()
293        },
294        BorderColor::all(Color::WHITE),
295        BackgroundColor(Color::BLACK),
296        children![
297            (
298                Node {
299                    border: UiRect::ZERO,
300                    width: percent(100),
301                    justify_content: JustifyContent::Center,
302                    align_items: AlignItems::Center,
303                    padding: UiRect::ZERO,
304                    margin: UiRect::ZERO,
305                    ..default()
306                },
307                BackgroundColor(Color::BLACK),
308                children![(
309                    Text::new(label),
310                    label_text_style.clone(),
311                    Node {
312                        margin: UiRect::vertical(px(3)),
313                        ..default()
314                    },
315                )]
316            ),
317            (
318                Node {
319                    width: percent(100),
320                    flex_direction: FlexDirection::Row,
321                    justify_content: JustifyContent::Center,
322                    align_items: AlignItems::Center,
323                    border: UiRect::top(px(1)),
324                    ..default()
325                },
326                BorderColor::all(Color::WHITE),
327                children![
328                    make_animation_label(true, AnimationLabel::Run),
329                    make_animation_label(false, AnimationLabel::Walk),
330                    make_animation_label(false, AnimationLabel::Idle),
331                    make_animation_label(false, AnimationLabel::Off),
332                ]
333            )
334        ],
335    )
336}
```

examples/ui/styling/borders.rs ([line 45](../../src/borders/borders.rs.html#45))

```rust
12fn setup(mut commands: Commands) {
13    commands.spawn(Camera2d);
14
15    // labels for the different border edges
16    let border_labels = [
17        "None",
18        "All",
19        "Left",
20        "Right",
21        "Top",
22        "Bottom",
23        "Horizontal",
24        "Vertical",
25        "Top Left",
26        "Bottom Left",
27        "Top Right",
28        "Bottom Right",
29        "Top Bottom Right",
30        "Top Bottom Left",
31        "Top Left Right",
32        "Bottom Left Right",
33    ];
34
35    // all the different combinations of border edges
36    // these correspond to the labels above
37    let borders = [
38        UiRect::default(),
39        UiRect::all(px(10)),
40        UiRect::left(px(10)),
41        UiRect::right(px(10)),
42        UiRect::top(px(10)),
43        UiRect::bottom(px(10)),
44        UiRect::horizontal(px(10)),
45        UiRect::vertical(px(10)),
46        UiRect {
47            left: px(20),
48            top: px(10),
49            ..default()
50        },
51        UiRect {
52            left: px(10),
53            bottom: px(20),
54            ..default()
55        },
56        UiRect {
57            right: px(20),
58            top: px(10),
59            ..default()
60        },
61        UiRect {
62            right: px(10),
63            bottom: px(10),
64            ..default()
65        },
66        UiRect {
67            right: px(10),
68            top: px(20),
69            bottom: px(10),
70            ..default()
71        },
72        UiRect {
73            left: px(10),
74            top: px(10),
75            bottom: px(10),
76            ..default()
77        },
78        UiRect {
79            left: px(20),
80            right: px(10),
81            top: px(10),
82            ..default()
83        },
84        UiRect {
85            left: px(10),
86            right: px(10),
87            bottom: px(20),
88            ..default()
89        },
90    ];
91
92    let borders_examples = (
93        Node {
94            margin: px(25).all(),
95            flex_wrap: FlexWrap::Wrap,
96            ..default()
97        },
98        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
99            |(label, border)| {
100                (
101                    Node {
102                        flex_direction: FlexDirection::Column,
103                        align_items: AlignItems::Center,
104                        ..default()
105                    },
106                    children![
107                        (
108                            Node {
109                                width: px(50),
110                                height: px(50),
111                                border,
112                                margin: px(20).all(),
113                                align_items: AlignItems::Center,
114                                justify_content: JustifyContent::Center,
115                                ..default()
116                            },
117                            BackgroundColor(MAROON.into()),
118                            BorderColor {
119                                top: RED.into(),
120                                bottom: YELLOW.into(),
121                                left: GREEN.into(),
122                                right: BLUE.into(),
123                            },
124                            Outline {
125                                width: px(6),
126                                offset: px(6),
127                                color: Color::WHITE,
128                            },
129                            children![(
130                                Node {
131                                    width: px(10),
132                                    height: px(10),
133                                    ..default()
134                                },
135                                BackgroundColor(YELLOW.into()),
136                            )]
137                        ),
138                        (Text::new(label), TextFont::from_font_size(9.0))
139                    ],
140                )
141            },
142        ))),
143    );
144
145    let non_zero = |x, y| x != px(0) && y != px(0);
146    let border_size = move |x, y| {
147        if non_zero(x, y) {
148            f32::MAX
149        } else {
150            0.
151        }
152    };
153
154    let borders_examples_rounded = (
155        Node {
156            margin: px(25).all(),
157            flex_wrap: FlexWrap::Wrap,
158            ..default()
159        },
160        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
161            move |(label, border)| {
162                (
163                    Node {
164                        flex_direction: FlexDirection::Column,
165                        align_items: AlignItems::Center,
166                        ..default()
167                    },
168                    children![
169                        (
170                            Node {
171                                width: px(50),
172                                height: px(50),
173                                border,
174                                margin: px(20).all(),
175                                align_items: AlignItems::Center,
176                                justify_content: JustifyContent::Center,
177                                border_radius: BorderRadius::px(
178                                    border_size(border.left, border.top),
179                                    border_size(border.right, border.top),
180                                    border_size(border.right, border.bottom,),
181                                    border_size(border.left, border.bottom),
182                                ),
183                                ..default()
184                            },
185                            BackgroundColor(MAROON.into()),
186                            BorderColor {
187                                top: RED.into(),
188                                bottom: YELLOW.into(),
189                                left: GREEN.into(),
190                                right: BLUE.into(),
191                            },
192                            Outline {
193                                width: px(6),
194                                offset: px(6),
195                                color: Color::WHITE,
196                            },
197                            children![(
198                                Node {
199                                    width: px(10),
200                                    height: px(10),
201                                    border_radius: BorderRadius::MAX,
202                                    ..default()
203                                },
204                                BackgroundColor(YELLOW.into()),
205                            )],
206                        ),
207                        (Text::new(label), TextFont::from_font_size(9.0))
208                    ],
209                )
210            },
211        ))),
212    );
213
214    commands.spawn((
215        Node {
216            margin: px(25).all(),
217            flex_direction: FlexDirection::Column,
218            align_self: AlignSelf::Stretch,
219            justify_self: JustifySelf::Stretch,
220            ..default()
221        },
222        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
223        children![
224            label("Borders"),
225            borders_examples,
226            label("Borders Rounded"),
227            borders_examples_rounded
228        ],
229    ));
230}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#807)

#### pub const fn [axes](#method.axes)(horizontal: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val"), vertical: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") where both `left` and `right` take the value of `horizontal`, and both `top` and `bottom` take the value of `vertical`.

##### Example

```rust
let ui_rect = UiRect::axes(Val::Px(10.0), Val::Percent(15.0));

assert_eq!(ui_rect.left, Val::Px(10.0));
assert_eq!(ui_rect.right, Val::Px(10.0));
assert_eq!(ui_rect.top, Val::Percent(15.0));
assert_eq!(ui_rect.bottom, Val::Percent(15.0));
```

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/3d/../helpers/widgets.rs ([line 42](../../src/clustered_decal_maps/helpers/widgets.rs.html#42))

```rust
42pub const BUTTON_PADDING: UiRect = UiRect::axes(Val::Px(12.0), Val::Px(6.0));
```

Hide additional examples

examples/ui/layout/flex\_layout.rs ([line 152](../../src/flex_layout/flex_layout.rs.html#152))

```rust
141fn spawn_nested_text_bundle(
142    builder: &mut ChildSpawnerCommands,
143    font: Handle<Font>,
144    background_color: Color,
145    margin: UiRect,
146    text: &str,
147) {
148    builder
149        .spawn((
150            Node {
151                margin,
152                padding: UiRect::axes(px(5), px(1)),
153                ..default()
154            },
155            BackgroundColor(background_color),
156        ))
157        .with_children(|builder| {
158            builder.spawn((Text::new(text), TextFont::from(font), TextColor::BLACK));
159        });
160}
```

examples/ui/layout/display\_and\_visibility.rs ([line 386](../../src/display_and_visibility/display_and_visibility.rs.html#386))

```rust
376fn spawn_button<T>(parent: &mut ChildSpawnerCommands, text_font: TextFont, target: Entity)
377where
378    T: Default + std::fmt::Debug + Send + Sync + 'static,
379    Target<T>: TargetUpdate,
380{
381    parent
382        .spawn((
383            Button,
384            Node {
385                align_self: AlignSelf::FlexStart,
386                padding: UiRect::axes(px(5), px(1)),
387                ..default()
388            },
389            BackgroundColor(Color::BLACK.with_alpha(0.5)),
390            Target::<T>::new(target),
391        ))
392        .with_children(|builder| {
393            builder.spawn((
394                Text(format!("{}::{:?}", Target::<T>::NAME, T::default())),
395                text_font,
396                TextLayout::justify(Justify::Center),
397            ));
398        });
399}
```

examples/ui/layout/anchor\_layout.rs ([line 133](../../src/anchor_layout/anchor_layout.rs.html#133))

```rust
119fn anchored_node(font: Handle<Font>, node: Node, label: &str) -> impl Bundle {
120    (
121        // outer gray box
122        Node {
123            grid_column: GridPlacement::span(1),
124            grid_row: GridPlacement::span(1),
125            ..default()
126        },
127        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
128        children![
129            // inner label box
130            (
131                Node {
132                    display: Display::Block,
133                    padding: UiRect::axes(px(5), px(1)),
134                    position_type: PositionType::Absolute,
135                    ..node
136                },
137                BackgroundColor(Color::srgb(1., 0.066, 0.349)),
138                children![(Text::new(label), TextFont::from(font), TextColor::BLACK,)],
139            )
140        ],
141    )
142}
```

examples/ui/widgets/standard\_widgets.rs ([line 235](../../src/standard_widgets/standard_widgets.rs.html#235))

```rust
222fn menu_button(asset_server: &AssetServer) -> impl Bundle {
223    (
224        Node { ..default() },
225        DemoMenuAnchor,
226        observe(on_menu_event),
227        children![(
228            Node {
229                width: px(200),
230                height: px(65),
231                border: UiRect::all(px(5)),
232                box_sizing: BoxSizing::BorderBox,
233                justify_content: JustifyContent::SpaceBetween,
234                align_items: AlignItems::Center,
235                padding: UiRect::axes(px(16), px(0)),
236                border_radius: BorderRadius::all(px(5)),
237                ..default()
238            },
239            DemoMenuButton,
240            Button,
241            MenuButton,
242            Hovered::default(),
243            TabIndex(0),
244            BorderColor::all(Color::BLACK),
245            BackgroundColor(NORMAL_BUTTON),
246            children![
247                (
248                    Text::new("Menu"),
249                    TextFont {
250                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
251                        font_size: FontSize::Px(33.0),
252                        ..default()
253                    },
254                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
255                    TextShadow::default(),
256                ),
257                (
258                    Node {
259                        width: px(12),
260                        height: px(12),
261                        ..default()
262                    },
263                    BackgroundColor(GRAY.into()),
264                )
265            ],
266        )],
267    )
268}
269
270fn update_button_style(
271    mut buttons: Query<
272        (
273            Has<Pressed>,
274            &Hovered,
275            Has<InteractionDisabled>,
276            &mut BackgroundColor,
277            &mut BorderColor,
278            &Children,
279        ),
280        (
281            Or<(
282                Changed<Pressed>,
283                Changed<Hovered>,
284                Added<InteractionDisabled>,
285            )>,
286            With<DemoButton>,
287        ),
288    >,
289    mut text_query: Query<&mut Text>,
290) {
291    for (pressed, hovered, disabled, mut color, mut border_color, children) in &mut buttons {
292        let mut text = text_query.get_mut(children[0]).unwrap();
293        set_button_style(
294            disabled,
295            hovered.get(),
296            pressed,
297            &mut color,
298            &mut border_color,
299            &mut text,
300        );
301    }
302}
303
304/// Supplementary system to detect removed marker components
305fn update_button_style2(
306    mut buttons: Query<
307        (
308            Has<Pressed>,
309            &Hovered,
310            Has<InteractionDisabled>,
311            &mut BackgroundColor,
312            &mut BorderColor,
313            &Children,
314        ),
315        With<DemoButton>,
316    >,
317    mut removed_depressed: RemovedComponents<Pressed>,
318    mut removed_disabled: RemovedComponents<InteractionDisabled>,
319    mut text_query: Query<&mut Text>,
320) {
321    removed_depressed
322        .read()
323        .chain(removed_disabled.read())
324        .for_each(|entity| {
325            if let Ok((pressed, hovered, disabled, mut color, mut border_color, children)) =
326                buttons.get_mut(entity)
327            {
328                let mut text = text_query.get_mut(children[0]).unwrap();
329                set_button_style(
330                    disabled,
331                    hovered.get(),
332                    pressed,
333                    &mut color,
334                    &mut border_color,
335                    &mut text,
336                );
337            }
338        });
339}
340
341fn set_button_style(
342    disabled: bool,
343    hovered: bool,
344    pressed: bool,
345    color: &mut BackgroundColor,
346    border_color: &mut BorderColor,
347    text: &mut Text,
348) {
349    match (disabled, hovered, pressed) {
350        // Disabled button
351        (true, _, _) => {
352            **text = "Disabled".to_string();
353            *color = NORMAL_BUTTON.into();
354            border_color.set_all(GRAY);
355        }
356
357        // Pressed and hovered button
358        (false, true, true) => {
359            **text = "Press".to_string();
360            *color = PRESSED_BUTTON.into();
361            border_color.set_all(RED);
362        }
363
364        // Hovered, unpressed button
365        (false, true, false) => {
366            **text = "Hover".to_string();
367            *color = HOVERED_BUTTON.into();
368            border_color.set_all(WHITE);
369        }
370
371        // Unhovered button (either pressed or not).
372        (false, false, _) => {
373            **text = "Button".to_string();
374            *color = NORMAL_BUTTON.into();
375            border_color.set_all(BLACK);
376        }
377    }
378}
379
380/// Create a demo slider
381fn slider(min: f32, max: f32, value: f32) -> impl Bundle {
382    (
383        Node {
384            display: Display::Flex,
385            flex_direction: FlexDirection::Column,
386            justify_content: JustifyContent::Center,
387            align_items: AlignItems::Stretch,
388            justify_items: JustifyItems::Center,
389            column_gap: px(4),
390            height: px(12),
391            width: percent(30),
392            ..default()
393        },
394        Name::new("Slider"),
395        Hovered::default(),
396        DemoSlider,
397        Slider {
398            track_click: TrackClick::Snap,
399            ..Default::default()
400        },
401        SliderValue(value),
402        SliderRange::new(min, max),
403        TabIndex(0),
404        Children::spawn((
405            // Slider background rail
406            Spawn((
407                Node {
408                    height: px(6),
409                    border_radius: BorderRadius::all(px(3)),
410                    ..default()
411                },
412                BackgroundColor(SLIDER_TRACK), // Border color for the slider
413            )),
414            // Invisible track to allow absolute placement of thumb entity. This is narrower than
415            // the actual slider, which allows us to position the thumb entity using simple
416            // percentages, without having to measure the actual width of the slider thumb.
417            Spawn((
418                Node {
419                    display: Display::Flex,
420                    position_type: PositionType::Absolute,
421                    left: px(0),
422                    // Track is short by 12px to accommodate the thumb.
423                    right: px(12),
424                    top: px(0),
425                    bottom: px(0),
426                    ..default()
427                },
428                children![(
429                    // Thumb
430                    DemoSliderThumb,
431                    SliderThumb,
432                    Node {
433                        display: Display::Flex,
434                        width: px(12),
435                        height: px(12),
436                        position_type: PositionType::Absolute,
437                        left: percent(0), // This will be updated by the slider's value
438                        border_radius: BorderRadius::MAX,
439                        ..default()
440                    },
441                    BackgroundColor(SLIDER_THUMB),
442                )],
443            )),
444        )),
445    )
446}
447
448/// Update the visuals of the slider based on the slider state.
449fn update_slider_style(
450    sliders: Query<
451        (
452            Entity,
453            &SliderValue,
454            &SliderRange,
455            &Hovered,
456            &SliderDragState,
457            Has<InteractionDisabled>,
458        ),
459        (
460            Or<(
461                Changed<SliderValue>,
462                Changed<SliderRange>,
463                Changed<Hovered>,
464                Changed<SliderDragState>,
465                Added<InteractionDisabled>,
466            )>,
467            With<DemoSlider>,
468        ),
469    >,
470    children: Query<&Children>,
471    mut thumbs: Query<(&mut Node, &mut BackgroundColor, Has<DemoSliderThumb>), Without<DemoSlider>>,
472) {
473    for (slider_ent, value, range, hovered, drag_state, disabled) in sliders.iter() {
474        for child in children.iter_descendants(slider_ent) {
475            if let Ok((mut thumb_node, mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
476                && is_thumb
477            {
478                thumb_node.left = percent(range.thumb_position(value.0) * 100.0);
479                thumb_bg.0 = thumb_color(disabled, hovered.0 | drag_state.dragging);
480            }
481        }
482    }
483}
484
485fn update_slider_style2(
486    sliders: Query<
487        (Entity, &Hovered, &SliderDragState, Has<InteractionDisabled>),
488        With<DemoSlider>,
489    >,
490    children: Query<&Children>,
491    mut thumbs: Query<(&mut BackgroundColor, Has<DemoSliderThumb>), Without<DemoSlider>>,
492    mut removed_disabled: RemovedComponents<InteractionDisabled>,
493) {
494    removed_disabled.read().for_each(|entity| {
495        if let Ok((slider_ent, hovered, drag_state, disabled)) = sliders.get(entity) {
496            for child in children.iter_descendants(slider_ent) {
497                if let Ok((mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
498                    && is_thumb
499                {
500                    thumb_bg.0 = thumb_color(disabled, hovered.0 | drag_state.dragging);
501                }
502            }
503        }
504    });
505}
506
507fn thumb_color(disabled: bool, hovered: bool) -> Color {
508    match (disabled, hovered) {
509        (true, _) => ELEMENT_FILL_DISABLED,
510
511        (false, true) => SLIDER_THUMB.lighter(0.3),
512
513        _ => SLIDER_THUMB,
514    }
515}
516
517/// Create a demo checkbox
518fn checkbox(asset_server: &AssetServer, caption: &str) -> impl Bundle {
519    (
520        Node {
521            display: Display::Flex,
522            flex_direction: FlexDirection::Row,
523            justify_content: JustifyContent::FlexStart,
524            align_items: AlignItems::Center,
525            align_content: AlignContent::Center,
526            column_gap: px(4),
527            ..default()
528        },
529        Name::new("Checkbox"),
530        Hovered::default(),
531        DemoCheckbox,
532        Checkbox,
533        TabIndex(0),
534        Children::spawn((
535            Spawn((
536                // Checkbox outer
537                Node {
538                    display: Display::Flex,
539                    width: px(16),
540                    height: px(16),
541                    border: UiRect::all(px(2)),
542                    border_radius: BorderRadius::all(px(3)),
543                    ..default()
544                },
545                BorderColor::all(ELEMENT_OUTLINE), // Border color for the checkbox
546                children![
547                    // Checkbox inner
548                    (
549                        Node {
550                            display: Display::Flex,
551                            width: px(8),
552                            height: px(8),
553                            position_type: PositionType::Absolute,
554                            left: px(2),
555                            top: px(2),
556                            ..default()
557                        },
558                        BackgroundColor(ELEMENT_FILL),
559                    ),
560                ],
561            )),
562            Spawn((
563                Text::new(caption),
564                TextFont {
565                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
566                    font_size: FontSize::Px(20.0),
567                    ..default()
568                },
569            )),
570        )),
571    )
572}
573
574// Update the element's styles.
575fn update_checkbox_or_radio_style(
576    mut q_checkbox: Query<
577        (Has<Checked>, &Hovered, Has<InteractionDisabled>, &Children),
578        (
579            Or<(With<DemoCheckbox>, With<DemoRadio>)>,
580            Or<(
581                Added<DemoCheckbox>,
582                Changed<Hovered>,
583                Added<Checked>,
584                Added<InteractionDisabled>,
585            )>,
586        ),
587    >,
588    mut q_border_color: Query<
589        (&mut BorderColor, &mut Children),
590        (Without<DemoCheckbox>, Without<DemoRadio>),
591    >,
592    mut q_bg_color: Query<&mut BackgroundColor, (Without<DemoCheckbox>, Without<Children>)>,
593) {
594    for (checked, Hovered(is_hovering), is_disabled, children) in q_checkbox.iter_mut() {
595        let Some(border_id) = children.first() else {
596            continue;
597        };
598
599        let Ok((mut border_color, border_children)) = q_border_color.get_mut(*border_id) else {
600            continue;
601        };
602
603        let Some(mark_id) = border_children.first() else {
604            warn!("Checkbox does not have a mark entity.");
605            continue;
606        };
607
608        let Ok(mut mark_bg) = q_bg_color.get_mut(*mark_id) else {
609            warn!("Checkbox mark entity lacking a background color.");
610            continue;
611        };
612
613        set_checkbox_or_radio_style(
614            is_disabled,
615            *is_hovering,
616            checked,
617            &mut border_color,
618            &mut mark_bg,
619        );
620    }
621}
622
623fn update_checkbox_or_radio_style2(
624    mut q_checkbox: Query<
625        (Has<Checked>, &Hovered, Has<InteractionDisabled>, &Children),
626        Or<(With<DemoCheckbox>, With<DemoRadio>)>,
627    >,
628    mut q_border_color: Query<
629        (&mut BorderColor, &mut Children),
630        (Without<DemoCheckbox>, Without<DemoRadio>),
631    >,
632    mut q_bg_color: Query<
633        &mut BackgroundColor,
634        (Without<DemoCheckbox>, Without<DemoRadio>, Without<Children>),
635    >,
636    mut removed_checked: RemovedComponents<Checked>,
637    mut removed_disabled: RemovedComponents<InteractionDisabled>,
638) {
639    removed_checked
640        .read()
641        .chain(removed_disabled.read())
642        .for_each(|entity| {
643            if let Ok((checked, Hovered(is_hovering), is_disabled, children)) =
644                q_checkbox.get_mut(entity)
645            {
646                let Some(border_id) = children.first() else {
647                    return;
648                };
649
650                let Ok((mut border_color, border_children)) = q_border_color.get_mut(*border_id)
651                else {
652                    return;
653                };
654
655                let Some(mark_id) = border_children.first() else {
656                    warn!("Checkbox does not have a mark entity.");
657                    return;
658                };
659
660                let Ok(mut mark_bg) = q_bg_color.get_mut(*mark_id) else {
661                    warn!("Checkbox mark entity lacking a background color.");
662                    return;
663                };
664
665                set_checkbox_or_radio_style(
666                    is_disabled,
667                    *is_hovering,
668                    checked,
669                    &mut border_color,
670                    &mut mark_bg,
671                );
672            }
673        });
674}
675
676fn set_checkbox_or_radio_style(
677    disabled: bool,
678    hovering: bool,
679    checked: bool,
680    border_color: &mut BorderColor,
681    mark_bg: &mut BackgroundColor,
682) {
683    let color: Color = if disabled {
684        // If the element is disabled, use a lighter color
685        ELEMENT_OUTLINE.with_alpha(0.2)
686    } else if hovering {
687        // If hovering, use a lighter color
688        ELEMENT_OUTLINE.lighter(0.2)
689    } else {
690        // Default color for the element
691        ELEMENT_OUTLINE
692    };
693
694    // Update the background color of the element
695    border_color.set_all(color);
696
697    let mark_color: Color = match (disabled, checked) {
698        (true, true) => ELEMENT_FILL_DISABLED,
699        (false, true) => ELEMENT_FILL,
700        (_, false) => Srgba::NONE.into(),
701    };
702
703    if mark_bg.0 != mark_color {
704        // Update the color of the element
705        mark_bg.0 = mark_color;
706    }
707}
708
709/// Create a demo radio group
710fn radio_group(asset_server: &AssetServer) -> impl Bundle {
711    (
712        Node {
713            display: Display::Flex,
714            flex_direction: FlexDirection::Column,
715            align_items: AlignItems::Start,
716            column_gap: px(4),
717            ..default()
718        },
719        Name::new("RadioGroup"),
720        RadioGroup,
721        TabIndex::default(),
722        children![
723            (radio(asset_server, TrackClick::Drag, "Slider Drag"),),
724            (radio(asset_server, TrackClick::Step, "Slider Step"),),
725            (radio(asset_server, TrackClick::Snap, "Slider Snap"),)
726        ],
727    )
728}
729
730/// Create a demo radio button
731fn radio(asset_server: &AssetServer, value: TrackClick, caption: &str) -> impl Bundle {
732    (
733        Node {
734            display: Display::Flex,
735            flex_direction: FlexDirection::Row,
736            justify_content: JustifyContent::FlexStart,
737            align_items: AlignItems::Center,
738            align_content: AlignContent::Center,
739            column_gap: px(4),
740            ..default()
741        },
742        Name::new("RadioButton"),
743        Hovered::default(),
744        DemoRadio(value),
745        RadioButton,
746        Children::spawn((
747            Spawn((
748                // Radio outer
749                Node {
750                    display: Display::Flex,
751                    width: px(16),
752                    height: px(16),
753                    border: UiRect::all(px(2)),
754                    border_radius: BorderRadius::MAX,
755                    ..default()
756                },
757                BorderColor::all(ELEMENT_OUTLINE), // Border color for the radio button
758                children![
759                    // Radio inner
760                    (
761                        Node {
762                            display: Display::Flex,
763                            width: px(8),
764                            height: px(8),
765                            position_type: PositionType::Absolute,
766                            left: px(2),
767                            top: px(2),
768                            border_radius: BorderRadius::MAX,
769                            ..default()
770                        },
771                        BackgroundColor(ELEMENT_FILL),
772                    ),
773                ],
774            )),
775            Spawn((
776                Text::new(caption),
777                TextFont {
778                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
779                    font_size: FontSize::Px(20.0),
780                    ..default()
781                },
782            )),
783        )),
784    )
785}
786
787fn on_menu_event(
788    menu_event: On<MenuEvent>,
789    q_anchor: Single<(Entity, &Children), With<DemoMenuAnchor>>,
790    q_popup: Query<Entity, With<MenuPopup>>,
791    assets: Res<AssetServer>,
792    mut focus: ResMut<InputFocus>,
793    mut commands: Commands,
794) {
795    let (anchor, children) = q_anchor.into_inner();
796    let popup = children.iter().find_map(|c| q_popup.get(c).ok());
797    info!("Menu action: {:?}", menu_event.action);
798    match menu_event.action {
799        MenuAction::Open(_) => {
800            if popup.is_none() {
801                spawn_menu(anchor, assets, commands);
802            }
803        }
804        MenuAction::Toggle => match popup {
805            Some(popup) => commands.entity(popup).despawn(),
806            None => spawn_menu(anchor, assets, commands),
807        },
808        MenuAction::CloseAll => {
809            if let Some(popup) = popup {
810                commands.entity(popup).despawn();
811            }
812        }
813        MenuAction::FocusRoot => {
814            focus.set(anchor, FocusCause::Navigated);
815        }
816    }
817}
818
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
868
869fn menu_item(asset_server: &AssetServer) -> impl Bundle {
870    (
871        Node {
872            padding: UiRect::axes(px(8), px(2)),
873            justify_content: JustifyContent::Center,
874            align_items: AlignItems::Start,
875            ..default()
876        },
877        DemoMenuItem,
878        MenuItem,
879        Hovered::default(),
880        TabIndex(0),
881        BackgroundColor(NORMAL_BUTTON),
882        children![(
883            Text::new("Menu Item"),
884            TextFont {
885                font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
886                font_size: FontSize::Px(33.0),
887                ..default()
888            },
889            TextColor(Color::srgb(0.9, 0.9, 0.9)),
890            TextShadow::default(),
891        )],
892    )
893}
```

examples/stress\_tests/many\_buttons.rs ([line 276](../../src/many_buttons/many_buttons.rs.html#276))

```rust
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
```

Additional examples can be found in:  

*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#246)
*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#1728)
*   [examples/ui/text/letter\_spacing.rs](../../src/letter_spacing/letter_spacing.rs.html#47)
*   [examples/ui/text/text\_debug.rs](../../src/text_debug/text_debug.rs.html#53)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#189)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#831)

#### pub const fn [left](#method.left)(left: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") where `left` takes the given value, and the other fields are set to `Val::ZERO`.

##### Example

```rust
let ui_rect = UiRect::left(Val::Px(10.0));

assert_eq!(ui_rect.left, Val::Px(10.0));
assert_eq!(ui_rect.right, Val::ZERO);
assert_eq!(ui_rect.top, Val::ZERO);
assert_eq!(ui_rect.bottom, Val::ZERO);
```

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/animation/animation\_graph.rs ([line 370](../../src/animation_graph/animation_graph.rs.html#370))

```rust
346fn setup_node_lines(commands: &mut Commands) {
347    for line in &HORIZONTAL_LINES {
348        commands.spawn((
349            Node {
350                position_type: PositionType::Absolute,
351                bottom: px(line.bottom),
352                left: px(line.left),
353                height: px(0),
354                width: px(line.length),
355                border: UiRect::bottom(px(1)),
356                ..default()
357            },
358            BorderColor::all(WHITE),
359        ));
360    }
361
362    for line in &VERTICAL_LINES {
363        commands.spawn((
364            Node {
365                position_type: PositionType::Absolute,
366                bottom: px(line.bottom),
367                left: px(line.left),
368                height: px(line.length),
369                width: px(0),
370                border: UiRect::left(px(1)),
371                ..default()
372            },
373            BorderColor::all(WHITE),
374        ));
375    }
376}
```

Hide additional examples

examples/testbed/ui.rs ([line 760](../../src/testbed_ui/ui.rs.html#760))

```rust
744    pub fn setup(mut commands: Commands) {
745        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Borders)));
746        let root = commands
747            .spawn((
748                Node {
749                    flex_wrap: FlexWrap::Wrap,
750                    ..default()
751                },
752                DespawnOnExit(super::Scene::Borders),
753            ))
754            .id();
755
756        // all the different combinations of border edges
757        let borders = [
758            UiRect::default(),
759            UiRect::all(px(20)),
760            UiRect::left(px(20)),
761            UiRect::vertical(px(20)),
762            UiRect {
763                left: px(40),
764                top: px(20),
765                ..Default::default()
766            },
767            UiRect {
768                right: px(20),
769                bottom: px(30),
770                ..Default::default()
771            },
772            UiRect {
773                right: px(20),
774                top: px(40),
775                bottom: px(20),
776                ..Default::default()
777            },
778            UiRect {
779                left: px(20),
780                top: px(20),
781                bottom: px(20),
782                ..Default::default()
783            },
784            UiRect {
785                left: px(20),
786                right: px(20),
787                bottom: px(40),
788                ..Default::default()
789            },
790        ];
791
792        let non_zero = |x, y| x != px(0) && y != px(0);
793        let border_size = |x, y| if non_zero(x, y) { f32::MAX } else { 0. };
794
795        for border in borders {
796            for rounded in [true, false] {
797                let border_node = commands
798                    .spawn((
799                        Node {
800                            width: px(100),
801                            height: px(100),
802                            border,
803                            margin: UiRect::all(px(30)),
804                            align_items: AlignItems::Center,
805                            justify_content: JustifyContent::Center,
806                            border_radius: if rounded {
807                                BorderRadius::px(
808                                    border_size(border.left, border.top),
809                                    border_size(border.right, border.top),
810                                    border_size(border.right, border.bottom),
811                                    border_size(border.left, border.bottom),
812                                )
813                            } else {
814                                BorderRadius::ZERO
815                            },
816                            ..default()
817                        },
818                        BackgroundColor(MAROON.into()),
819                        BorderColor::all(RED),
820                        Outline {
821                            width: px(10),
822                            offset: px(10),
823                            color: Color::WHITE,
824                        },
825                    ))
826                    .id();
827
828                commands.entity(root).add_child(border_node);
829            }
830        }
831    }
832}
833
834mod box_shadow {
835    use bevy::{color::palettes::css::*, prelude::*};
836
837    pub fn setup(mut commands: Commands) {
838        commands.spawn((Camera2d, DespawnOnExit(super::Scene::BoxShadow)));
839
840        commands
841            .spawn((
842                Node {
843                    width: percent(100),
844                    height: percent(100),
845                    padding: UiRect::all(px(30)),
846                    column_gap: px(200),
847                    flex_wrap: FlexWrap::Wrap,
848                    ..default()
849                },
850                BackgroundColor(GREEN.into()),
851                DespawnOnExit(super::Scene::BoxShadow),
852            ))
853            .with_children(|commands| {
854                let example_nodes = [
855                    (
856                        Vec2::splat(100.),
857                        Vec2::ZERO,
858                        10.,
859                        0.,
860                        BorderRadius::bottom_right(px(10)),
861                    ),
862                    (Vec2::new(200., 50.), Vec2::ZERO, 10., 0., BorderRadius::MAX),
863                    (
864                        Vec2::new(100., 50.),
865                        Vec2::ZERO,
866                        10.,
867                        10.,
868                        BorderRadius::ZERO,
869                    ),
870                    (
871                        Vec2::splat(100.),
872                        Vec2::splat(20.),
873                        10.,
874                        10.,
875                        BorderRadius::bottom_right(px(10)),
876                    ),
877                    (
878                        Vec2::splat(100.),
879                        Vec2::splat(50.),
880                        0.,
881                        10.,
882                        BorderRadius::ZERO,
883                    ),
884                    (
885                        Vec2::new(50., 100.),
886                        Vec2::splat(10.),
887                        0.,
888                        10.,
889                        BorderRadius::MAX,
890                    ),
891                ];
892
893                for (size, offset, spread, blur, border_radius) in example_nodes {
894                    commands.spawn((
895                        Node {
896                            width: px(size.x),
897                            height: px(size.y),
898                            border: UiRect::all(px(2)),
899                            border_radius,
900                            ..default()
901                        },
902                        BorderColor::all(WHITE),
903                        BackgroundColor(BLUE.into()),
904                        BoxShadow::new(
905                            Color::BLACK.with_alpha(0.9),
906                            percent(offset.x),
907                            percent(offset.y),
908                            percent(spread),
909                            px(blur),
910                        ),
911                    ));
912                }
913            });
914    }
915}
916
917mod text_wrap {
918    use bevy::prelude::*;
919
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
1710
1711    pub fn teardown(mut debug_options: ResMut<GlobalUiDebugOptions>) {
1712        *debug_options = GlobalUiDebugOptions::default();
1713    }
1714}
1715
1716mod viewport_coords {
1717    use bevy::{color::palettes::css::*, prelude::*};
1718
1719    const PALETTE: [Srgba; 9] = [RED, WHITE, BEIGE, AQUA, CRIMSON, NAVY, AZURE, LIME, BLACK];
1720
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
1799}
1800
1801mod outer_color {
1802    use bevy::prelude::*;
1803
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

examples/animation/animation\_masks.rs ([line 253](../../src/animation_masks/animation_masks.rs.html#253))

```rust
229fn new_mask_group_control(label: &str, width: Val, mask_group_id: u32) -> impl Bundle {
230    let button_text_style = (
231        TextFont {
232            font_size: FontSize::Px(14.0),
233            ..default()
234        },
235        TextColor::WHITE,
236    );
237    let selected_button_text_style = (button_text_style.0.clone(), TextColor::BLACK);
238    let label_text_style = (
239        button_text_style.0.clone(),
240        TextColor(Color::Srgba(LIGHT_GRAY)),
241    );
242
243    let make_animation_label = {
244        let button_text_style = button_text_style.clone();
245        let selected_button_text_style = selected_button_text_style.clone();
246        move |first: bool, label: AnimationLabel| {
247            (
248                Button,
249                BackgroundColor(if !first { Color::BLACK } else { Color::WHITE }),
250                Node {
251                    flex_grow: 1.0,
252                    border: if !first {
253                        UiRect::left(px(1))
254                    } else {
255                        UiRect::ZERO
256                    },
257                    ..default()
258                },
259                BorderColor::all(Color::WHITE),
260                AnimationControl {
261                    group_id: mask_group_id,
262                    label,
263                },
264                children![(
265                    Text(format!("{label:?}")),
266                    if !first {
267                        button_text_style.clone()
268                    } else {
269                        selected_button_text_style.clone()
270                    },
271                    TextLayout::justify(Justify::Center),
272                    Node {
273                        flex_grow: 1.0,
274                        margin: UiRect::vertical(px(3)),
275                        ..default()
276                    },
277                )],
278            )
279        }
280    };
281
282    (
283        Node {
284            border: UiRect::all(px(1)),
285            width,
286            flex_direction: FlexDirection::Column,
287            justify_content: JustifyContent::Center,
288            align_items: AlignItems::Center,
289            padding: UiRect::ZERO,
290            margin: UiRect::ZERO,
291            border_radius: BorderRadius::all(px(3)),
292            ..default()
293        },
294        BorderColor::all(Color::WHITE),
295        BackgroundColor(Color::BLACK),
296        children![
297            (
298                Node {
299                    border: UiRect::ZERO,
300                    width: percent(100),
301                    justify_content: JustifyContent::Center,
302                    align_items: AlignItems::Center,
303                    padding: UiRect::ZERO,
304                    margin: UiRect::ZERO,
305                    ..default()
306                },
307                BackgroundColor(Color::BLACK),
308                children![(
309                    Text::new(label),
310                    label_text_style.clone(),
311                    Node {
312                        margin: UiRect::vertical(px(3)),
313                        ..default()
314                    },
315                )]
316            ),
317            (
318                Node {
319                    width: percent(100),
320                    flex_direction: FlexDirection::Row,
321                    justify_content: JustifyContent::Center,
322                    align_items: AlignItems::Center,
323                    border: UiRect::top(px(1)),
324                    ..default()
325                },
326                BorderColor::all(Color::WHITE),
327                children![
328                    make_animation_label(true, AnimationLabel::Run),
329                    make_animation_label(false, AnimationLabel::Walk),
330                    make_animation_label(false, AnimationLabel::Idle),
331                    make_animation_label(false, AnimationLabel::Off),
332                ]
333            )
334        ],
335    )
336}
```

examples/ui/styling/box\_shadow.rs ([line 333](../../src/box_shadow/box_shadow.rs.html#333))

```rust
290fn build_setting_row(
291    setting_type: SettingType,
292    dec: SettingsButton,
293    inc: SettingsButton,
294    value: f32,
295    asset_server: &Res<AssetServer>,
296) -> impl Bundle {
297    let value_text = match setting_type {
298        SettingType::Shape => SHAPES[value as usize % SHAPES.len()].0.to_string(),
299        SettingType::Count => format!("{}", value as usize),
300        _ => format!("{value:.1}"),
301    };
302
303    (
304        Node {
305            flex_direction: FlexDirection::Row,
306            align_items: AlignItems::Center,
307            height: px(32),
308            ..default()
309        },
310        children![
311            (
312                Node {
313                    width: px(80),
314                    justify_content: JustifyContent::FlexEnd,
315                    align_items: AlignItems::Center,
316                    ..default()
317                },
318                // Attach SettingType to the value label node, not the parent row
319                children![(
320                    Text::new(setting_type.label()),
321                    TextFont {
322                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
323                        font_size: FontSize::Px(16.0),
324                        ..default()
325                    },
326                )],
327            ),
328            (
329                Button,
330                Node {
331                    width: px(28),
332                    height: px(28),
333                    margin: UiRect::left(px(8)),
334                    justify_content: JustifyContent::Center,
335                    align_items: AlignItems::Center,
336                    border_radius: BorderRadius::all(px(6)),
337                    ..default()
338                },
339                BackgroundColor(Color::WHITE),
340                dec,
341                children![(
342                    Text::new(if setting_type == SettingType::Shape {
343                        "<"
344                    } else {
345                        "-"
346                    }),
347                    TextFont {
348                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
349                        font_size: FontSize::Px(18.0),
350                        ..default()
351                    },
352                )],
353            ),
354            (
355                Node {
356                    width: px(48),
357                    height: px(28),
358                    margin: UiRect::horizontal(px(8)),
359                    justify_content: JustifyContent::Center,
360                    align_items: AlignItems::Center,
361                    border_radius: BorderRadius::all(px(6)),
362                    ..default()
363                },
364                children![{
365                    (
366                        Text::new(value_text),
367                        TextFont {
368                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
369                            font_size: FontSize::Px(16.0),
370                            ..default()
371                        },
372                        setting_type,
373                    )
374                }],
375            ),
376            (
377                Button,
378                Node {
379                    width: px(28),
380                    height: px(28),
381                    justify_content: JustifyContent::Center,
382                    align_items: AlignItems::Center,
383                    border_radius: BorderRadius::all(px(6)),
384                    ..default()
385                },
386                BackgroundColor(Color::WHITE),
387                inc,
388                children![(
389                    Text::new(if setting_type == SettingType::Shape {
390                        ">"
391                    } else {
392                        "+"
393                    }),
394                    TextFont {
395                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
396                        font_size: FontSize::Px(18.0),
397                        ..default()
398                    },
399                )],
400            ),
401        ],
402    )
403}
```

examples/ui/styling/borders.rs ([line 40](../../src/borders/borders.rs.html#40))

```rust
12fn setup(mut commands: Commands) {
13    commands.spawn(Camera2d);
14
15    // labels for the different border edges
16    let border_labels = [
17        "None",
18        "All",
19        "Left",
20        "Right",
21        "Top",
22        "Bottom",
23        "Horizontal",
24        "Vertical",
25        "Top Left",
26        "Bottom Left",
27        "Top Right",
28        "Bottom Right",
29        "Top Bottom Right",
30        "Top Bottom Left",
31        "Top Left Right",
32        "Bottom Left Right",
33    ];
34
35    // all the different combinations of border edges
36    // these correspond to the labels above
37    let borders = [
38        UiRect::default(),
39        UiRect::all(px(10)),
40        UiRect::left(px(10)),
41        UiRect::right(px(10)),
42        UiRect::top(px(10)),
43        UiRect::bottom(px(10)),
44        UiRect::horizontal(px(10)),
45        UiRect::vertical(px(10)),
46        UiRect {
47            left: px(20),
48            top: px(10),
49            ..default()
50        },
51        UiRect {
52            left: px(10),
53            bottom: px(20),
54            ..default()
55        },
56        UiRect {
57            right: px(20),
58            top: px(10),
59            ..default()
60        },
61        UiRect {
62            right: px(10),
63            bottom: px(10),
64            ..default()
65        },
66        UiRect {
67            right: px(10),
68            top: px(20),
69            bottom: px(10),
70            ..default()
71        },
72        UiRect {
73            left: px(10),
74            top: px(10),
75            bottom: px(10),
76            ..default()
77        },
78        UiRect {
79            left: px(20),
80            right: px(10),
81            top: px(10),
82            ..default()
83        },
84        UiRect {
85            left: px(10),
86            right: px(10),
87            bottom: px(20),
88            ..default()
89        },
90    ];
91
92    let borders_examples = (
93        Node {
94            margin: px(25).all(),
95            flex_wrap: FlexWrap::Wrap,
96            ..default()
97        },
98        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
99            |(label, border)| {
100                (
101                    Node {
102                        flex_direction: FlexDirection::Column,
103                        align_items: AlignItems::Center,
104                        ..default()
105                    },
106                    children![
107                        (
108                            Node {
109                                width: px(50),
110                                height: px(50),
111                                border,
112                                margin: px(20).all(),
113                                align_items: AlignItems::Center,
114                                justify_content: JustifyContent::Center,
115                                ..default()
116                            },
117                            BackgroundColor(MAROON.into()),
118                            BorderColor {
119                                top: RED.into(),
120                                bottom: YELLOW.into(),
121                                left: GREEN.into(),
122                                right: BLUE.into(),
123                            },
124                            Outline {
125                                width: px(6),
126                                offset: px(6),
127                                color: Color::WHITE,
128                            },
129                            children![(
130                                Node {
131                                    width: px(10),
132                                    height: px(10),
133                                    ..default()
134                                },
135                                BackgroundColor(YELLOW.into()),
136                            )]
137                        ),
138                        (Text::new(label), TextFont::from_font_size(9.0))
139                    ],
140                )
141            },
142        ))),
143    );
144
145    let non_zero = |x, y| x != px(0) && y != px(0);
146    let border_size = move |x, y| {
147        if non_zero(x, y) {
148            f32::MAX
149        } else {
150            0.
151        }
152    };
153
154    let borders_examples_rounded = (
155        Node {
156            margin: px(25).all(),
157            flex_wrap: FlexWrap::Wrap,
158            ..default()
159        },
160        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
161            move |(label, border)| {
162                (
163                    Node {
164                        flex_direction: FlexDirection::Column,
165                        align_items: AlignItems::Center,
166                        ..default()
167                    },
168                    children![
169                        (
170                            Node {
171                                width: px(50),
172                                height: px(50),
173                                border,
174                                margin: px(20).all(),
175                                align_items: AlignItems::Center,
176                                justify_content: JustifyContent::Center,
177                                border_radius: BorderRadius::px(
178                                    border_size(border.left, border.top),
179                                    border_size(border.right, border.top),
180                                    border_size(border.right, border.bottom,),
181                                    border_size(border.left, border.bottom),
182                                ),
183                                ..default()
184                            },
185                            BackgroundColor(MAROON.into()),
186                            BorderColor {
187                                top: RED.into(),
188                                bottom: YELLOW.into(),
189                                left: GREEN.into(),
190                                right: BLUE.into(),
191                            },
192                            Outline {
193                                width: px(6),
194                                offset: px(6),
195                                color: Color::WHITE,
196                            },
197                            children![(
198                                Node {
199                                    width: px(10),
200                                    height: px(10),
201                                    border_radius: BorderRadius::MAX,
202                                    ..default()
203                                },
204                                BackgroundColor(YELLOW.into()),
205                            )],
206                        ),
207                        (Text::new(label), TextFont::from_font_size(9.0))
208                    ],
209                )
210            },
211        ))),
212    );
213
214    commands.spawn((
215        Node {
216            margin: px(25).all(),
217            flex_direction: FlexDirection::Column,
218            align_self: AlignSelf::Stretch,
219            justify_self: JustifySelf::Stretch,
220            ..default()
221        },
222        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
223        children![
224            label("Borders"),
225            borders_examples,
226            label("Borders Rounded"),
227            borders_examples_rounded
228        ],
229    ));
230}
```

examples/ui/styling/gradients.rs ([line 117](../../src/gradients/gradients.rs.html#117))

```rust
27fn setup(mut commands: Commands) {
28    commands.spawn(Camera2d);
29
30    commands
31        .spawn(Node {
32            flex_direction: FlexDirection::Column,
33            row_gap: px(20),
34            margin: UiRect::all(px(20)),
35            ..Default::default()
36        })
37        .with_children(|commands| {
38            for (b, stops) in [
39                (
40                    4.,
41                    vec![
42                        ColorStop::new(Color::WHITE, percent(15)),
43                        ColorStop::new(Color::BLACK, percent(85)),
44                    ],
45                ),
46                (4., vec![RED.into(), BLUE.into(), LIME.into()]),
47                (
48                    0.,
49                    vec![
50                        RED.into(),
51                        ColorStop::new(RED, percent(100. / 7.)),
52                        ColorStop::new(ORANGE, percent(100. / 7.)),
53                        ColorStop::new(ORANGE, percent(200. / 7.)),
54                        ColorStop::new(YELLOW, percent(200. / 7.)),
55                        ColorStop::new(YELLOW, percent(300. / 7.)),
56                        ColorStop::new(GREEN, percent(300. / 7.)),
57                        ColorStop::new(GREEN, percent(400. / 7.)),
58                        ColorStop::new(BLUE, percent(400. / 7.)),
59                        ColorStop::new(BLUE, percent(500. / 7.)),
60                        ColorStop::new(INDIGO, percent(500. / 7.)),
61                        ColorStop::new(INDIGO, percent(600. / 7.)),
62                        ColorStop::new(VIOLET, percent(600. / 7.)),
63                        VIOLET.into(),
64                    ],
65                ),
66            ] {
67                commands.spawn(Node::default()).with_children(|commands| {
68                    commands
69                        .spawn(Node {
70                            flex_direction: FlexDirection::Column,
71                            row_gap: px(5),
72                            ..Default::default()
73                        })
74                        .with_children(|commands| {
75                            for (w, h) in [(70., 70.), (35., 70.), (70., 35.)] {
76                                commands
77                                    .spawn(Node {
78                                        column_gap: px(10),
79                                        ..Default::default()
80                                    })
81                                    .with_children(|commands| {
82                                        for angle in (0..8).map(|i| i as f32 * TAU / 8.) {
83                                            commands.spawn((
84                                                Node {
85                                                    width: px(w),
86                                                    height: px(h),
87                                                    border: UiRect::all(px(b)),
88                                                    border_radius: BorderRadius::all(px(20)),
89                                                    ..default()
90                                                },
91                                                BackgroundGradient::from(LinearGradient {
92                                                    angle,
93                                                    stops: stops.clone(),
94                                                    ..default()
95                                                }),
96                                                BorderGradient::from(LinearGradient {
97                                                    angle: 3. * TAU / 8.,
98                                                    stops: vec![
99                                                        YELLOW.into(),
100                                                        Color::WHITE.into(),
101                                                        ORANGE.into(),
102                                                    ],
103                                                    ..default()
104                                                }),
105                                            ));
106                                        }
107                                    });
108                            }
109                        });
110
111                    commands.spawn(Node::default()).with_children(|commands| {
112                        commands.spawn((
113                            Node {
114                                aspect_ratio: Some(1.),
115                                height: percent(100),
116                                border: UiRect::all(px(b)),
117                                margin: UiRect::left(px(20)),
118                                border_radius: BorderRadius::all(px(20)),
119                                ..default()
120                            },
121                            BackgroundGradient::from(LinearGradient {
122                                angle: 0.,
123                                stops: stops.clone(),
124                                ..default()
125                            }),
126                            BorderGradient::from(LinearGradient {
127                                angle: 3. * TAU / 8.,
128                                stops: vec![YELLOW.into(), Color::WHITE.into(), ORANGE.into()],
129                                ..default()
130                            }),
131                            AnimateMarker,
132                        ));
133
134                        commands.spawn((
135                            Node {
136                                aspect_ratio: Some(1.),
137                                height: percent(100),
138                                border: UiRect::all(px(b)),
139                                margin: UiRect::left(px(20)),
140                                border_radius: BorderRadius::all(px(20)),
141                                ..default()
142                            },
143                            BackgroundGradient::from(RadialGradient {
144                                stops: stops.clone(),
145                                shape: RadialGradientShape::ClosestSide,
146                                position: UiPosition::CENTER,
147                                ..default()
148                            }),
149                            BorderGradient::from(LinearGradient {
150                                angle: 3. * TAU / 8.,
151                                stops: vec![YELLOW.into(), Color::WHITE.into(), ORANGE.into()],
152                                ..default()
153                            }),
154                            AnimateMarker,
155                        ));
156                        commands.spawn((
157                            Node {
158                                aspect_ratio: Some(1.),
159                                height: percent(100),
160                                border: UiRect::all(px(b)),
161                                margin: UiRect::left(px(20)),
162                                border_radius: BorderRadius::all(px(20)),
163                                ..default()
164                            },
165                            BackgroundGradient::from(ConicGradient {
166                                start: 0.,
167                                stops: stops
168                                    .iter()
169                                    .map(|stop| AngularColorStop::auto(stop.color))
170                                    .collect(),
171                                position: UiPosition::CENTER,
172                                ..default()
173                            }),
174                            BorderGradient::from(LinearGradient {
175                                angle: 3. * TAU / 8.,
176                                stops: vec![YELLOW.into(), Color::WHITE.into(), ORANGE.into()],
177                                ..default()
178                            }),
179                            AnimateMarker,
180                        ));
181                    });
182                });
183            }
184
185            let button = commands.spawn((
186                        Button,
187                        Node {
188                            border: UiRect::all(px(2)),
189                            padding: UiRect::axes(px(8), px(4)),
190                            // horizontally center child text
191                            justify_content: JustifyContent::Center,
192                            // vertically center child text
193                            align_items: AlignItems::Center,
194                            border_radius: BorderRadius::MAX,
195                            ..default()
196                        },
197                        BorderColor::all(Color::WHITE),
198                        BackgroundColor(Color::BLACK),
199                        children![(
200                            Text::new("next color space"),
201                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
202                            TextShadow::default(),
203                        )]
204                )).observe(
205                    |_event: On<Pointer<Over>>, mut border_query: Query<&mut BorderColor, With<Button>>| {
206                    *border_query.single_mut().unwrap() = BorderColor::all(RED);
207
208
209                })
210                .observe(
211                    |_event: On<Pointer<Out>>, mut border_query: Query<&mut BorderColor, With<Button>>| {
212                    *border_query.single_mut().unwrap() = BorderColor::all(Color::WHITE);
213                })
214                .observe(
215                        |_event: On<Pointer<Click>>,
216                            mut gradients_query: Query<&mut BackgroundGradient>,
217                            mut label_query: Query<
218                            &mut Text,
219                            With<CurrentColorSpaceLabel>,
220                        >| {
221                            let mut current_space = InterpolationColorSpace::default();
222                            for mut gradients in gradients_query.iter_mut() {
223                                for gradient in gradients.0.iter_mut() {
224                                    let space = match gradient {
225                                        Gradient::Linear(linear_gradient) => {
226                                            &mut linear_gradient.color_space
227                                        }
228                                        Gradient::Radial(radial_gradient) => {
229                                            &mut radial_gradient.color_space
230                                        }
231                                        Gradient::Conic(conic_gradient) => {
232                                            &mut conic_gradient.color_space
233                                        }
234                                    };
235                                    *space = match *space {
236                                        InterpolationColorSpace::Oklaba => {
237                                            InterpolationColorSpace::Oklcha
238                                        }
239                                        InterpolationColorSpace::Oklcha => {
240                                            InterpolationColorSpace::OklchaLong
241                                        }
242                                        InterpolationColorSpace::OklchaLong => {
243                                            InterpolationColorSpace::Srgba
244                                        }
245                                        InterpolationColorSpace::Srgba => {
246                                            InterpolationColorSpace::LinearRgba
247                                        }
248                                        InterpolationColorSpace::LinearRgba => {
249                                            InterpolationColorSpace::Hsla
250                                        }
251                                        InterpolationColorSpace::Hsla => {
252                                            InterpolationColorSpace::HslaLong
253                                        }
254                                        InterpolationColorSpace::HslaLong => {
255                                            InterpolationColorSpace::Hsva
256                                        }
257                                        InterpolationColorSpace::Hsva => {
258                                            InterpolationColorSpace::HsvaLong
259                                        }
260                                        InterpolationColorSpace::HsvaLong => {
261                                            InterpolationColorSpace::Oklaba
262                                        }
263                                    };
264                                    current_space = *space;
265                                }
266                            }
267                            for mut label in label_query.iter_mut() {
268                                label.0 = format!("{current_space:?}");
269                            }
270                        }
271                    ).id();
272
273            commands.spawn(
274                Node {
275                    flex_direction: FlexDirection::Column,
276                    row_gap: px(10),
277                    align_items: AlignItems::Center,
278                    ..Default::default()
279                }
280            ).with_children(|commands| {
281                commands.spawn((Text::new(format!("{:?}", InterpolationColorSpace::default())), TextFont { font_size: FontSize::Px(25.), ..default() }, CurrentColorSpaceLabel));
282
283            })
284            .add_child(button);
285        });
286}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#853)

#### pub const fn [right](#method.right)(right: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") where `right` takes the given value, and the other fields are set to `Val::ZERO`.

##### Example

```rust
let ui_rect = UiRect::right(Val::Px(10.0));

assert_eq!(ui_rect.left, Val::ZERO);
assert_eq!(ui_rect.right, Val::Px(10.0));
assert_eq!(ui_rect.top, Val::ZERO);
assert_eq!(ui_rect.bottom, Val::ZERO);
```

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/3d/color\_grading.rs ([line 247](../../src/color_grading/color_grading.rs.html#247))

```rust
228fn button_for_value(
229    option: SelectedColorGradingOption,
230    color_grading: &ColorGrading,
231    font: &Handle<Font>,
232) -> impl Bundle {
233    let label = match option {
234        SelectedColorGradingOption::Global(option) => option.to_string(),
235        SelectedColorGradingOption::Section(_, option) => option.to_string(),
236    };
237
238    // Add the button node.
239    (
240        Button,
241        Node {
242            border: UiRect::all(px(1)),
243            width: px(200),
244            justify_content: JustifyContent::Center,
245            align_items: AlignItems::Center,
246            padding: UiRect::axes(px(12), px(6)),
247            margin: UiRect::right(px(12)),
248            border_radius: BorderRadius::MAX,
249            ..default()
250        },
251        BorderColor::all(Color::WHITE),
252        BackgroundColor(Color::BLACK),
253        ColorGradingOptionWidget {
254            widget_type: ColorGradingOptionWidgetType::Button,
255            option,
256        },
257        children![
258            // Add the button label.
259            (
260                text(&label, font, Color::WHITE),
261                ColorGradingOptionWidget {
262                    widget_type: ColorGradingOptionWidgetType::Label,
263                    option,
264                },
265            ),
266            // Add a spacer.
267            Node {
268                flex_grow: 1.0,
269                ..default()
270            },
271            // Add the value text.
272            (
273                text(
274                    &format!("{:.3}", option.get(color_grading)),
275                    font,
276                    Color::WHITE,
277                ),
278                ColorGradingOptionWidget {
279                    widget_type: ColorGradingOptionWidgetType::Value,
280                    option,
281                },
282            ),
283        ],
284    )
285}
```

Hide additional examples

examples/testbed/ui.rs ([line 1771](../../src/testbed_ui/ui.rs.html#1771))

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
1799}
1800
1801mod outer_color {
1802    use bevy::prelude::*;
1803
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

examples/ui/styling/borders.rs ([line 41](../../src/borders/borders.rs.html#41))

```rust
12fn setup(mut commands: Commands) {
13    commands.spawn(Camera2d);
14
15    // labels for the different border edges
16    let border_labels = [
17        "None",
18        "All",
19        "Left",
20        "Right",
21        "Top",
22        "Bottom",
23        "Horizontal",
24        "Vertical",
25        "Top Left",
26        "Bottom Left",
27        "Top Right",
28        "Bottom Right",
29        "Top Bottom Right",
30        "Top Bottom Left",
31        "Top Left Right",
32        "Bottom Left Right",
33    ];
34
35    // all the different combinations of border edges
36    // these correspond to the labels above
37    let borders = [
38        UiRect::default(),
39        UiRect::all(px(10)),
40        UiRect::left(px(10)),
41        UiRect::right(px(10)),
42        UiRect::top(px(10)),
43        UiRect::bottom(px(10)),
44        UiRect::horizontal(px(10)),
45        UiRect::vertical(px(10)),
46        UiRect {
47            left: px(20),
48            top: px(10),
49            ..default()
50        },
51        UiRect {
52            left: px(10),
53            bottom: px(20),
54            ..default()
55        },
56        UiRect {
57            right: px(20),
58            top: px(10),
59            ..default()
60        },
61        UiRect {
62            right: px(10),
63            bottom: px(10),
64            ..default()
65        },
66        UiRect {
67            right: px(10),
68            top: px(20),
69            bottom: px(10),
70            ..default()
71        },
72        UiRect {
73            left: px(10),
74            top: px(10),
75            bottom: px(10),
76            ..default()
77        },
78        UiRect {
79            left: px(20),
80            right: px(10),
81            top: px(10),
82            ..default()
83        },
84        UiRect {
85            left: px(10),
86            right: px(10),
87            bottom: px(20),
88            ..default()
89        },
90    ];
91
92    let borders_examples = (
93        Node {
94            margin: px(25).all(),
95            flex_wrap: FlexWrap::Wrap,
96            ..default()
97        },
98        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
99            |(label, border)| {
100                (
101                    Node {
102                        flex_direction: FlexDirection::Column,
103                        align_items: AlignItems::Center,
104                        ..default()
105                    },
106                    children![
107                        (
108                            Node {
109                                width: px(50),
110                                height: px(50),
111                                border,
112                                margin: px(20).all(),
113                                align_items: AlignItems::Center,
114                                justify_content: JustifyContent::Center,
115                                ..default()
116                            },
117                            BackgroundColor(MAROON.into()),
118                            BorderColor {
119                                top: RED.into(),
120                                bottom: YELLOW.into(),
121                                left: GREEN.into(),
122                                right: BLUE.into(),
123                            },
124                            Outline {
125                                width: px(6),
126                                offset: px(6),
127                                color: Color::WHITE,
128                            },
129                            children![(
130                                Node {
131                                    width: px(10),
132                                    height: px(10),
133                                    ..default()
134                                },
135                                BackgroundColor(YELLOW.into()),
136                            )]
137                        ),
138                        (Text::new(label), TextFont::from_font_size(9.0))
139                    ],
140                )
141            },
142        ))),
143    );
144
145    let non_zero = |x, y| x != px(0) && y != px(0);
146    let border_size = move |x, y| {
147        if non_zero(x, y) {
148            f32::MAX
149        } else {
150            0.
151        }
152    };
153
154    let borders_examples_rounded = (
155        Node {
156            margin: px(25).all(),
157            flex_wrap: FlexWrap::Wrap,
158            ..default()
159        },
160        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
161            move |(label, border)| {
162                (
163                    Node {
164                        flex_direction: FlexDirection::Column,
165                        align_items: AlignItems::Center,
166                        ..default()
167                    },
168                    children![
169                        (
170                            Node {
171                                width: px(50),
172                                height: px(50),
173                                border,
174                                margin: px(20).all(),
175                                align_items: AlignItems::Center,
176                                justify_content: JustifyContent::Center,
177                                border_radius: BorderRadius::px(
178                                    border_size(border.left, border.top),
179                                    border_size(border.right, border.top),
180                                    border_size(border.right, border.bottom,),
181                                    border_size(border.left, border.bottom),
182                                ),
183                                ..default()
184                            },
185                            BackgroundColor(MAROON.into()),
186                            BorderColor {
187                                top: RED.into(),
188                                bottom: YELLOW.into(),
189                                left: GREEN.into(),
190                                right: BLUE.into(),
191                            },
192                            Outline {
193                                width: px(6),
194                                offset: px(6),
195                                color: Color::WHITE,
196                            },
197                            children![(
198                                Node {
199                                    width: px(10),
200                                    height: px(10),
201                                    border_radius: BorderRadius::MAX,
202                                    ..default()
203                                },
204                                BackgroundColor(YELLOW.into()),
205                            )],
206                        ),
207                        (Text::new(label), TextFont::from_font_size(9.0))
208                    ],
209                )
210            },
211        ))),
212    );
213
214    commands.spawn((
215        Node {
216            margin: px(25).all(),
217            flex_direction: FlexDirection::Column,
218            align_self: AlignSelf::Stretch,
219            justify_self: JustifySelf::Stretch,
220            ..default()
221        },
222        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
223        children![
224            label("Borders"),
225            borders_examples,
226            label("Borders Rounded"),
227            borders_examples_rounded
228        ],
229    ));
230}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#875)

#### pub const fn [top](#method.top)(top: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") where `top` takes the given value, and the other fields are set to `Val::ZERO`.

##### Example

```rust
let ui_rect = UiRect::top(Val::Px(10.0));

assert_eq!(ui_rect.left, Val::ZERO);
assert_eq!(ui_rect.right, Val::ZERO);
assert_eq!(ui_rect.top, Val::Px(10.0));
assert_eq!(ui_rect.bottom, Val::ZERO);
```

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1823](../../src/testbed_ui/ui.rs.html#1823))

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

examples/ui/layout/size\_constraints.rs ([line 91](../../src/size_constraints/size_constraints.rs.html#91))

```rust
41fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
42    // ui camera
43    commands.spawn(Camera2d);
44
45    let text_font = (
46        TextFont {
47            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
48            font_size: FontSize::Px(33.0),
49            ..Default::default()
50        },
51        TextColor(Color::srgb(0.9, 0.9, 0.9)),
52    );
53
54    commands
55        .spawn((
56            Node {
57                width: percent(100),
58                height: percent(100),
59                justify_content: JustifyContent::Center,
60                align_items: AlignItems::Center,
61                ..default()
62            },
63            BackgroundColor(Color::BLACK),
64        ))
65        .with_children(|parent| {
66            parent
67                .spawn(Node {
68                    flex_direction: FlexDirection::Column,
69                    align_items: AlignItems::Center,
70                    justify_content: JustifyContent::Center,
71                    ..default()
72                })
73                .with_children(|parent| {
74                    parent.spawn((
75                        Text::new("Size Constraints Example"),
76                        text_font.clone(),
77                        Node {
78                            margin: UiRect::bottom(px(25)),
79                            ..Default::default()
80                        },
81                    ));
82
83                    spawn_bar(parent);
84
85                    parent
86                        .spawn((
87                            Node {
88                                flex_direction: FlexDirection::Column,
89                                align_items: AlignItems::Stretch,
90                                padding: UiRect::all(px(10)),
91                                margin: UiRect::top(px(50)),
92                                ..default()
93                            },
94                            BackgroundColor(YELLOW.into()),
95                        ))
96                        .with_children(|parent| {
97                            for constraint in [
98                                Constraint::MinWidth,
99                                Constraint::FlexBasis,
100                                Constraint::Width,
101                                Constraint::MaxWidth,
102                            ] {
103                                spawn_button_row(parent, constraint, text_font.clone());
104                            }
105                        });
106                });
107        });
108}
```

examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs ([line 58](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#58))

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

examples/animation/animation\_masks.rs ([line 323](../../src/animation_masks/animation_masks.rs.html#323))

```rust
229fn new_mask_group_control(label: &str, width: Val, mask_group_id: u32) -> impl Bundle {
230    let button_text_style = (
231        TextFont {
232            font_size: FontSize::Px(14.0),
233            ..default()
234        },
235        TextColor::WHITE,
236    );
237    let selected_button_text_style = (button_text_style.0.clone(), TextColor::BLACK);
238    let label_text_style = (
239        button_text_style.0.clone(),
240        TextColor(Color::Srgba(LIGHT_GRAY)),
241    );
242
243    let make_animation_label = {
244        let button_text_style = button_text_style.clone();
245        let selected_button_text_style = selected_button_text_style.clone();
246        move |first: bool, label: AnimationLabel| {
247            (
248                Button,
249                BackgroundColor(if !first { Color::BLACK } else { Color::WHITE }),
250                Node {
251                    flex_grow: 1.0,
252                    border: if !first {
253                        UiRect::left(px(1))
254                    } else {
255                        UiRect::ZERO
256                    },
257                    ..default()
258                },
259                BorderColor::all(Color::WHITE),
260                AnimationControl {
261                    group_id: mask_group_id,
262                    label,
263                },
264                children![(
265                    Text(format!("{label:?}")),
266                    if !first {
267                        button_text_style.clone()
268                    } else {
269                        selected_button_text_style.clone()
270                    },
271                    TextLayout::justify(Justify::Center),
272                    Node {
273                        flex_grow: 1.0,
274                        margin: UiRect::vertical(px(3)),
275                        ..default()
276                    },
277                )],
278            )
279        }
280    };
281
282    (
283        Node {
284            border: UiRect::all(px(1)),
285            width,
286            flex_direction: FlexDirection::Column,
287            justify_content: JustifyContent::Center,
288            align_items: AlignItems::Center,
289            padding: UiRect::ZERO,
290            margin: UiRect::ZERO,
291            border_radius: BorderRadius::all(px(3)),
292            ..default()
293        },
294        BorderColor::all(Color::WHITE),
295        BackgroundColor(Color::BLACK),
296        children![
297            (
298                Node {
299                    border: UiRect::ZERO,
300                    width: percent(100),
301                    justify_content: JustifyContent::Center,
302                    align_items: AlignItems::Center,
303                    padding: UiRect::ZERO,
304                    margin: UiRect::ZERO,
305                    ..default()
306                },
307                BackgroundColor(Color::BLACK),
308                children![(
309                    Text::new(label),
310                    label_text_style.clone(),
311                    Node {
312                        margin: UiRect::vertical(px(3)),
313                        ..default()
314                    },
315                )]
316            ),
317            (
318                Node {
319                    width: percent(100),
320                    flex_direction: FlexDirection::Row,
321                    justify_content: JustifyContent::Center,
322                    align_items: AlignItems::Center,
323                    border: UiRect::top(px(1)),
324                    ..default()
325                },
326                BorderColor::all(Color::WHITE),
327                children![
328                    make_animation_label(true, AnimationLabel::Run),
329                    make_animation_label(false, AnimationLabel::Walk),
330                    make_animation_label(false, AnimationLabel::Idle),
331                    make_animation_label(false, AnimationLabel::Off),
332                ]
333            )
334        ],
335    )
336}
```

examples/ui/styling/box\_shadow.rs ([line 259](../../src/box_shadow/box_shadow.rs.html#259))

```rust
139fn setup(
140    mut commands: Commands,
141    asset_server: Res<AssetServer>,
142    shadow: Res<ShadowSettings>,
143    shape: Res<ShapeSettings>,
144) {
145    commands.spawn((Camera2d, BoxShadowSamples(shadow.samples)));
146    // Spawn shape node
147    commands
148        .spawn((
149            Node {
150                width: percent(100),
151                height: percent(100),
152                align_items: AlignItems::Center,
153                justify_content: JustifyContent::Center,
154                ..default()
155            },
156            BackgroundColor(GRAY.into()),
157        ))
158        .insert(children![{
159            let mut node = Node {
160                width: px(164),
161                height: px(164),
162                border: UiRect::all(px(1)),
163                align_items: AlignItems::Center,
164                justify_content: JustifyContent::Center,
165                border_radius: BorderRadius::ZERO,
166                ..default()
167            };
168            SHAPES[shape.index % SHAPES.len()].1(&mut node);
169
170            (
171                node,
172                BorderColor::all(WHITE),
173                BackgroundColor(Color::srgb(0.21, 0.21, 0.21)),
174                BoxShadow(vec![ShadowStyle {
175                    color: Color::BLACK.with_alpha(0.8),
176                    x_offset: px(shadow.x_offset),
177                    y_offset: px(shadow.y_offset),
178                    spread_radius: px(shadow.spread),
179                    blur_radius: px(shadow.blur),
180                }]),
181                ShadowNode,
182            )
183        }]);
184
185    // Settings Panel
186    commands
187        .spawn((
188            Node {
189                flex_direction: FlexDirection::Column,
190                position_type: PositionType::Absolute,
191                left: px(24),
192                bottom: px(24),
193                width: px(270),
194                padding: UiRect::all(px(16)),
195                border_radius: BorderRadius::all(px(12)),
196                ..default()
197            },
198            BackgroundColor(Color::srgb(0.12, 0.12, 0.12).with_alpha(0.85)),
199            BorderColor::all(Color::WHITE.with_alpha(0.15)),
200            ZIndex(10),
201        ))
202        .insert(children![
203            build_setting_row(
204                SettingType::Shape,
205                SettingsButton::ShapePrev,
206                SettingsButton::ShapeNext,
207                shape.index as f32,
208                &asset_server,
209            ),
210            build_setting_row(
211                SettingType::XOffset,
212                SettingsButton::XOffsetDec,
213                SettingsButton::XOffsetInc,
214                shadow.x_offset,
215                &asset_server,
216            ),
217            build_setting_row(
218                SettingType::YOffset,
219                SettingsButton::YOffsetDec,
220                SettingsButton::YOffsetInc,
221                shadow.y_offset,
222                &asset_server,
223            ),
224            build_setting_row(
225                SettingType::Blur,
226                SettingsButton::BlurDec,
227                SettingsButton::BlurInc,
228                shadow.blur,
229                &asset_server,
230            ),
231            build_setting_row(
232                SettingType::Spread,
233                SettingsButton::SpreadDec,
234                SettingsButton::SpreadInc,
235                shadow.spread,
236                &asset_server,
237            ),
238            build_setting_row(
239                SettingType::Count,
240                SettingsButton::CountDec,
241                SettingsButton::CountInc,
242                shadow.count as f32,
243                &asset_server,
244            ),
245            // Add BoxShadowSamples as a setting row
246            build_setting_row(
247                SettingType::Samples,
248                SettingsButton::SamplesDec,
249                SettingsButton::SamplesInc,
250                shadow.samples as f32,
251                &asset_server,
252            ),
253            // Reset button
254            (
255                Node {
256                    flex_direction: FlexDirection::Row,
257                    align_items: AlignItems::Center,
258                    height: px(36),
259                    margin: UiRect::top(px(12)),
260                    ..default()
261                },
262                children![(
263                    Button,
264                    Node {
265                        width: px(90),
266                        height: px(32),
267                        justify_content: JustifyContent::Center,
268                        align_items: AlignItems::Center,
269                        border_radius: BorderRadius::all(px(8)),
270                        ..default()
271                    },
272                    BackgroundColor(NORMAL_BUTTON),
273                    SettingsButton::Reset,
274                    children![(
275                        Text::new("Reset"),
276                        TextFont {
277                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
278                            font_size: FontSize::Px(16.0),
279                            ..default()
280                        },
281                    )],
282                )],
283            ),
284        ]);
285}
```

examples/ui/styling/borders.rs ([line 42](../../src/borders/borders.rs.html#42))

```rust
12fn setup(mut commands: Commands) {
13    commands.spawn(Camera2d);
14
15    // labels for the different border edges
16    let border_labels = [
17        "None",
18        "All",
19        "Left",
20        "Right",
21        "Top",
22        "Bottom",
23        "Horizontal",
24        "Vertical",
25        "Top Left",
26        "Bottom Left",
27        "Top Right",
28        "Bottom Right",
29        "Top Bottom Right",
30        "Top Bottom Left",
31        "Top Left Right",
32        "Bottom Left Right",
33    ];
34
35    // all the different combinations of border edges
36    // these correspond to the labels above
37    let borders = [
38        UiRect::default(),
39        UiRect::all(px(10)),
40        UiRect::left(px(10)),
41        UiRect::right(px(10)),
42        UiRect::top(px(10)),
43        UiRect::bottom(px(10)),
44        UiRect::horizontal(px(10)),
45        UiRect::vertical(px(10)),
46        UiRect {
47            left: px(20),
48            top: px(10),
49            ..default()
50        },
51        UiRect {
52            left: px(10),
53            bottom: px(20),
54            ..default()
55        },
56        UiRect {
57            right: px(20),
58            top: px(10),
59            ..default()
60        },
61        UiRect {
62            right: px(10),
63            bottom: px(10),
64            ..default()
65        },
66        UiRect {
67            right: px(10),
68            top: px(20),
69            bottom: px(10),
70            ..default()
71        },
72        UiRect {
73            left: px(10),
74            top: px(10),
75            bottom: px(10),
76            ..default()
77        },
78        UiRect {
79            left: px(20),
80            right: px(10),
81            top: px(10),
82            ..default()
83        },
84        UiRect {
85            left: px(10),
86            right: px(10),
87            bottom: px(20),
88            ..default()
89        },
90    ];
91
92    let borders_examples = (
93        Node {
94            margin: px(25).all(),
95            flex_wrap: FlexWrap::Wrap,
96            ..default()
97        },
98        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
99            |(label, border)| {
100                (
101                    Node {
102                        flex_direction: FlexDirection::Column,
103                        align_items: AlignItems::Center,
104                        ..default()
105                    },
106                    children![
107                        (
108                            Node {
109                                width: px(50),
110                                height: px(50),
111                                border,
112                                margin: px(20).all(),
113                                align_items: AlignItems::Center,
114                                justify_content: JustifyContent::Center,
115                                ..default()
116                            },
117                            BackgroundColor(MAROON.into()),
118                            BorderColor {
119                                top: RED.into(),
120                                bottom: YELLOW.into(),
121                                left: GREEN.into(),
122                                right: BLUE.into(),
123                            },
124                            Outline {
125                                width: px(6),
126                                offset: px(6),
127                                color: Color::WHITE,
128                            },
129                            children![(
130                                Node {
131                                    width: px(10),
132                                    height: px(10),
133                                    ..default()
134                                },
135                                BackgroundColor(YELLOW.into()),
136                            )]
137                        ),
138                        (Text::new(label), TextFont::from_font_size(9.0))
139                    ],
140                )
141            },
142        ))),
143    );
144
145    let non_zero = |x, y| x != px(0) && y != px(0);
146    let border_size = move |x, y| {
147        if non_zero(x, y) {
148            f32::MAX
149        } else {
150            0.
151        }
152    };
153
154    let borders_examples_rounded = (
155        Node {
156            margin: px(25).all(),
157            flex_wrap: FlexWrap::Wrap,
158            ..default()
159        },
160        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
161            move |(label, border)| {
162                (
163                    Node {
164                        flex_direction: FlexDirection::Column,
165                        align_items: AlignItems::Center,
166                        ..default()
167                    },
168                    children![
169                        (
170                            Node {
171                                width: px(50),
172                                height: px(50),
173                                border,
174                                margin: px(20).all(),
175                                align_items: AlignItems::Center,
176                                justify_content: JustifyContent::Center,
177                                border_radius: BorderRadius::px(
178                                    border_size(border.left, border.top),
179                                    border_size(border.right, border.top),
180                                    border_size(border.right, border.bottom,),
181                                    border_size(border.left, border.bottom),
182                                ),
183                                ..default()
184                            },
185                            BackgroundColor(MAROON.into()),
186                            BorderColor {
187                                top: RED.into(),
188                                bottom: YELLOW.into(),
189                                left: GREEN.into(),
190                                right: BLUE.into(),
191                            },
192                            Outline {
193                                width: px(6),
194                                offset: px(6),
195                                color: Color::WHITE,
196                            },
197                            children![(
198                                Node {
199                                    width: px(10),
200                                    height: px(10),
201                                    border_radius: BorderRadius::MAX,
202                                    ..default()
203                                },
204                                BackgroundColor(YELLOW.into()),
205                            )],
206                        ),
207                        (Text::new(label), TextFont::from_font_size(9.0))
208                    ],
209                )
210            },
211        ))),
212    );
213
214    commands.spawn((
215        Node {
216            margin: px(25).all(),
217            flex_direction: FlexDirection::Column,
218            align_self: AlignSelf::Stretch,
219            justify_self: JustifySelf::Stretch,
220            ..default()
221        },
222        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
223        children![
224            label("Borders"),
225            borders_examples,
226            label("Borders Rounded"),
227            borders_examples_rounded
228        ],
229    ));
230}
```

Additional examples can be found in:  

*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#354)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#897)

#### pub const fn [bottom](#method.bottom)(bottom: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Creates a new [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") where `bottom` takes the given value, and the other fields are set to `Val::ZERO`.

##### Example

```rust
let ui_rect = UiRect::bottom(Val::Px(10.0));

assert_eq!(ui_rect.left, Val::ZERO);
assert_eq!(ui_rect.right, Val::ZERO);
assert_eq!(ui_rect.top, Val::ZERO);
assert_eq!(ui_rect.bottom, Val::Px(10.0));
```

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/animation/animation\_graph.rs ([line 355](../../src/animation_graph/animation_graph.rs.html#355))

```rust
346fn setup_node_lines(commands: &mut Commands) {
347    for line in &HORIZONTAL_LINES {
348        commands.spawn((
349            Node {
350                position_type: PositionType::Absolute,
351                bottom: px(line.bottom),
352                left: px(line.left),
353                height: px(0),
354                width: px(line.length),
355                border: UiRect::bottom(px(1)),
356                ..default()
357            },
358            BorderColor::all(WHITE),
359        ));
360    }
361
362    for line in &VERTICAL_LINES {
363        commands.spawn((
364            Node {
365                position_type: PositionType::Absolute,
366                bottom: px(line.bottom),
367                left: px(line.left),
368                height: px(line.length),
369                width: px(0),
370                border: UiRect::left(px(1)),
371                ..default()
372            },
373            BorderColor::all(WHITE),
374        ));
375    }
376}
```

Hide additional examples

examples/ui/relative\_cursor\_position.rs ([line 42](../../src/relative_cursor_position/relative_cursor_position.rs.html#42))

```rust
13fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
14    commands.spawn((
15        Camera2d,
16        Camera {
17            // Cursor position will take the viewport offset into account
18            viewport: Some(Viewport {
19                physical_position: [200, 100].into(),
20                physical_size: [600, 600].into(),
21                ..default()
22            }),
23            ..default()
24        },
25    ));
26
27    commands
28        .spawn(Node {
29            width: percent(100),
30            height: percent(100),
31            align_items: AlignItems::Center,
32            justify_content: JustifyContent::Center,
33            flex_direction: FlexDirection::Column,
34            ..default()
35        })
36        .with_children(|parent| {
37            parent
38                .spawn((
39                    Node {
40                        width: px(250),
41                        height: px(250),
42                        margin: UiRect::bottom(px(15)),
43                        ..default()
44                    },
45                    BackgroundColor(Color::srgb(0.92, 0.14, 0.05)),
46                ))
47                .insert(RelativeCursorPosition::default());
48
49            parent.spawn((
50                Text::new("(0.0, 0.0)"),
51                TextFont {
52                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
53                    font_size: FontSize::Px(33.0),
54                    ..default()
55                },
56                TextColor(Color::srgb(0.9, 0.9, 0.9)),
57            ));
58        });
59}
```

examples/testbed/ui.rs ([line 1829](../../src/testbed_ui/ui.rs.html#1829))

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

examples/ui/layout/size\_constraints.rs ([line 78](../../src/size_constraints/size_constraints.rs.html#78))

```rust
41fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
42    // ui camera
43    commands.spawn(Camera2d);
44
45    let text_font = (
46        TextFont {
47            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
48            font_size: FontSize::Px(33.0),
49            ..Default::default()
50        },
51        TextColor(Color::srgb(0.9, 0.9, 0.9)),
52    );
53
54    commands
55        .spawn((
56            Node {
57                width: percent(100),
58                height: percent(100),
59                justify_content: JustifyContent::Center,
60                align_items: AlignItems::Center,
61                ..default()
62            },
63            BackgroundColor(Color::BLACK),
64        ))
65        .with_children(|parent| {
66            parent
67                .spawn(Node {
68                    flex_direction: FlexDirection::Column,
69                    align_items: AlignItems::Center,
70                    justify_content: JustifyContent::Center,
71                    ..default()
72                })
73                .with_children(|parent| {
74                    parent.spawn((
75                        Text::new("Size Constraints Example"),
76                        text_font.clone(),
77                        Node {
78                            margin: UiRect::bottom(px(25)),
79                            ..Default::default()
80                        },
81                    ));
82
83                    spawn_bar(parent);
84
85                    parent
86                        .spawn((
87                            Node {
88                                flex_direction: FlexDirection::Column,
89                                align_items: AlignItems::Stretch,
90                                padding: UiRect::all(px(10)),
91                                margin: UiRect::top(px(50)),
92                                ..default()
93                            },
94                            BackgroundColor(YELLOW.into()),
95                        ))
96                        .with_children(|parent| {
97                            for constraint in [
98                                Constraint::MinWidth,
99                                Constraint::FlexBasis,
100                                Constraint::Width,
101                                Constraint::MaxWidth,
102                            ] {
103                                spawn_button_row(parent, constraint, text_font.clone());
104                            }
105                        });
106                });
107        });
108}
```

examples/ui/text/generic\_font\_families.rs ([line 51](../../src/generic_font_families/generic_font_families.rs.html#51))

```rust
33fn setup(mut commands: Commands, mut font_system: ResMut<FontCx>) {
34    // UI camera
35    commands.spawn(Camera2d);
36
37    commands
38        .spawn((Node {
39            display: Display::Grid,
40            grid_template_columns: vec![RepeatedGridTrack::fr(3, 1.)],
41            margin: UiRect::AUTO,
42            row_gap: px(25),
43            column_gap: px(15),
44            ..Default::default()
45        },))
46        .with_children(|builder| {
47            builder.spawn((
48                Node {
49                    justify_self: JustifySelf::Center,
50                    grid_column: GridPlacement::span(3),
51                    margin: UiRect::bottom(px(15)),
52                    ..default()
53                },
54                Text::new("Generic Font Families"),
55                TextFont::from_font_size(FONT_SIZE),
56                Underline,
57            ));
58
59            let outline = Outline {
60                color: ZINC_600.into(),
61                width: px(2.),
62                offset: px(4.),
63            };
64
65            for (source, description) in [
66                (FontSource::SansSerif, "generic sans serif font"),
67                (FontSource::Serif, "generic serif font"),
68                (FontSource::Fantasy, "generic fantasy font"),
69                (FontSource::Cursive, "generic cursive font"),
70                (FontSource::Monospace, "generic monospace font"),
71            ] {
72                builder.spawn((
73                    Text::new(description),
74                    TextFont::from(source.clone()).with_font_size(FONT_SIZE),
75                    TextColor(WHEAT.into()),
76                    TextLayout::justify(Justify::Center),
77                    outline,
78                ));
79
80                builder.spawn((
81                    Text::new(format!("FontSource::{source:?}")),
82                    TextFont::from_font_size(FONT_SIZE),
83                    TextColor(YELLOW.into()),
84                    TextLayout::justify(Justify::Center),
85                    outline,
86                ));
87
88                // Get the family name for the `FontSource` from `FontCx`.
89                // `get_family` only returns `None` for `FontSource::Handle`.
90                let family_name = font_system.get_family(&source).unwrap();
91                builder.spawn((
92                    Text::new(family_name),
93                    TextFont::from_font_size(FONT_SIZE),
94                    TextLayout::justify(Justify::Center),
95                    outline,
96                ));
97            }
98        });
99}
```

examples/ui/scroll\_and\_overflow/overflow.rs ([line 51](../../src/overflow/overflow.rs.html#51))

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

Additional examples can be found in:  

*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#48)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#101)
*   [examples/ui/styling/borders.rs](../../src/borders/borders.rs.html#43)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#918)

#### pub const fn [with\_left](#method.with_left)(self, left: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Returns the [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") with its `left` field set to the given value.

##### Example

```rust
let ui_rect = UiRect::all(Val::Px(20.0)).with_left(Val::Px(10.0));
assert_eq!(ui_rect.left, Val::Px(10.0));
assert_eq!(ui_rect.right, Val::Px(20.0));
assert_eq!(ui_rect.top, Val::Px(20.0));
assert_eq!(ui_rect.bottom, Val::Px(20.0));
```

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

examples/3d/ssr.rs ([line 574](../../src/ssr/ssr.rs.html#574))

```rust
555fn range_controls(
556    value: f32,
557    marker: RangeValueText,
558    dec_setting: ExampleSetting,
559    inc_setting: ExampleSetting,
560) -> impl Bundle {
561    (
562        Node {
563            align_items: AlignItems::Center,
564            ..default()
565        },
566        Children::spawn((
567            Spawn(adjustment_button(dec_setting, "<", Some(true))),
568            Spawn((
569                Node {
570                    width: px(50),
571                    height: px(33),
572                    justify_content: JustifyContent::Center,
573                    align_items: AlignItems::Center,
574                    border: BUTTON_BORDER.with_left(px(0)).with_right(px(0)),
575                    ..default()
576                },
577                BackgroundColor(Color::WHITE),
578                BUTTON_BORDER_COLOR,
579                marker,
580                children![(widgets::ui_text(&format!("{:.2}", value), Color::BLACK))],
581            )),
582            Spawn(adjustment_button(inc_setting, ">", Some(false))),
583        )),
584    )
585}
586
587fn adjustment_button(
588    setting: ExampleSetting,
589    label: &str,
590    is_left_right: Option<bool>,
591) -> impl Bundle {
592    (
593        Button,
594        Node {
595            height: px(33),
596            border: if let Some(is_left) = is_left_right {
597                if is_left {
598                    BUTTON_BORDER.with_right(px(0))
599                } else {
600                    BUTTON_BORDER.with_left(px(0))
601                }
602            } else {
603                BUTTON_BORDER
604            },
605            justify_content: JustifyContent::Center,
606            align_items: AlignItems::Center,
607            padding: BUTTON_PADDING,
608            border_radius: match is_left_right {
609                Some(true) => BorderRadius::ZERO.with_left(BUTTON_BORDER_RADIUS_SIZE),
610                Some(false) => BorderRadius::ZERO.with_right(BUTTON_BORDER_RADIUS_SIZE),
611                None => BorderRadius::all(BUTTON_BORDER_RADIUS_SIZE),
612            },
613            ..default()
614        },
615        BUTTON_BORDER_COLOR,
616        BackgroundColor(Color::BLACK),
617        RadioButton,
618        WidgetClickSender(setting),
619        children![(widgets::ui_text(label, Color::WHITE), RadioButtonText)],
620    )
621}
```

Hide additional examples

examples/3d/../helpers/widgets.rs ([line 82](../../src/clustered_decal_maps/helpers/widgets.rs.html#82))

```rust
62pub fn option_button<T>(
63    option_value: T,
64    option_name: &str,
65    is_selected: bool,
66    is_first: bool,
67    is_last: bool,
68) -> impl Bundle
69where
70    T: Clone + Send + Sync + 'static,
71{
72    let (bg_color, fg_color) = if is_selected {
73        (Color::WHITE, Color::BLACK)
74    } else {
75        (Color::BLACK, Color::WHITE)
76    };
77
78    // Add the button node.
79    (
80        Button,
81        Node {
82            border: BUTTON_BORDER.with_left(if is_first { px(1) } else { px(0) }),
83            justify_content: JustifyContent::Center,
84            align_items: AlignItems::Center,
85            padding: BUTTON_PADDING,
86            border_radius: BorderRadius::ZERO
87                .with_left(if is_first {
88                    BUTTON_BORDER_RADIUS_SIZE
89                } else {
90                    px(0)
91                })
92                .with_right(if is_last {
93                    BUTTON_BORDER_RADIUS_SIZE
94                } else {
95                    px(0)
96                }),
97            ..default()
98        },
99        BUTTON_BORDER_COLOR,
100        BackgroundColor(bg_color),
101        RadioButton,
102        WidgetClickSender(option_value.clone()),
103        children![(
104            ui_text(option_name, fg_color),
105            RadioButtonText,
106            WidgetClickSender(option_value),
107        )],
108    )
109}
```

examples/testbed/ui.rs ([line 1997](../../src/testbed_ui/ui.rs.html#1997))

```rust
1854    pub fn setup(mut commands: Commands) {
1855        commands.spawn((Camera2d, DespawnOnExit(super::Scene::BoxedContent)));
1856        commands
1857            .spawn((
1858                Node {
1859                    margin: auto().all(),
1860                    column_gap: px(30),
1861                    ..default()
1862                },
1863                DespawnOnExit(super::Scene::BoxedContent),
1864            ))
1865            .with_children(|builder| {
1866                for (heading, text_justify) in [
1867                    ("Left", Justify::Left),
1868                    ("Center", Justify::Center),
1869                    ("Right", Justify::Right),
1870                ] {
1871                    builder
1872                        .spawn(Node {
1873                            flex_direction: FlexDirection::Column,
1874                            align_items: AlignItems::Center,
1875                            justify_content: JustifyContent::Start,
1876                            row_gap: px(20),
1877                            ..default()
1878                        })
1879                        .with_children(|builder| {
1880                            builder.spawn((
1881                                Node::default(),
1882                                Text::new(format!("{heading} justify")),
1883                                TextFont::from_font_size(FontSize::Px(14.)),
1884                                TextLayout::justify(Justify::Center),
1885                            ));
1886
1887                            builder.spawn((
1888                                Node::default(),
1889                                Text::new("This text has\nno border or padding."),
1890                                TextFont::from_font_size(FontSize::Px(10.)),
1891                                TextLayout::justify(text_justify),
1892                                Outline {
1893                                    width: px(2),
1894                                    color: Color::WHITE,
1895                                    ..Default::default()
1896                                },
1897                            ));
1898
1899                            builder.spawn((
1900                                Node {
1901                                    border: px(10).all(),
1902                                    ..default()
1903                                },
1904                                Text::new("This text has\na border but no padding."),
1905                                TextFont::from_font_size(FontSize::Px(10.)),
1906                                TextLayout::justify(text_justify),
1907                                BorderColor::all(RED),
1908                                Outline {
1909                                    width: px(2),
1910                                    color: Color::WHITE,
1911                                    ..Default::default()
1912                                },
1913                            ));
1914
1915                            builder.spawn((
1916                                Node {
1917                                    padding: px(20).all(),
1918                                    ..default()
1919                                },
1920                                Text::new("This text has\npadding but no border."),
1921                                TextFont::from_font_size(FontSize::Px(10.)),
1922                                TextLayout::justify(text_justify),
1923                                Outline {
1924                                    width: px(2),
1925                                    color: Color::WHITE,
1926                                    ..Default::default()
1927                                },
1928                            ));
1929
1930                            builder.spawn((
1931                                Node {
1932                                    border: px(10).all(),
1933                                    padding: px(20).all(),
1934                                    ..default()
1935                                },
1936                                Text::new("This text has\nborder and padding."),
1937                                TextFont::from_font_size(FontSize::Px(10.)),
1938                                TextLayout::justify(text_justify),
1939                                BorderColor::all(RED),
1940                                Outline {
1941                                    width: px(2),
1942                                    color: Color::WHITE,
1943                                    ..Default::default()
1944                                },
1945                            ));
1946
1947                            builder.spawn((
1948                                Node {
1949                                    border: px(10).left(),
1950                                    ..default()
1951                                },
1952                                Text::new("This text has\na left border and no padding."),
1953                                TextFont::from_font_size(FontSize::Px(10.)),
1954                                TextLayout::justify(text_justify),
1955                                BorderColor::all(RED),
1956                                Outline {
1957                                    width: px(2),
1958                                    color: Color::WHITE,
1959                                    ..Default::default()
1960                                },
1961                            ));
1962
1963                            builder.spawn((
1964                                Node {
1965                                    border: px(10).right(),
1966                                    ..default()
1967                                },
1968                                Text::new("This text has\na right border and no padding."),
1969                                TextFont::from_font_size(FontSize::Px(10.)),
1970                                TextLayout::justify(text_justify),
1971                                BorderColor::all(RED),
1972                                Outline {
1973                                    width: px(2),
1974                                    color: Color::WHITE,
1975                                    ..Default::default()
1976                                },
1977                            ));
1978
1979                            builder.spawn((
1980                                Node {
1981                                    padding: px(20).top().with_right(px(20)),
1982                                    ..default()
1983                                },
1984                                Text::new("This text has\npadding on its top and right."),
1985                                TextFont::from_font_size(FontSize::Px(10.)),
1986                                TextLayout::justify(text_justify),
1987                                BorderColor::all(RED),
1988                                Outline {
1989                                    width: px(2),
1990                                    color: Color::WHITE,
1991                                    ..Default::default()
1992                                },
1993                            ));
1994
1995                            builder.spawn((
1996                                Node {
1997                                    padding: px(20).bottom().with_left(px(20)),
1998                                    ..default()
1999                                },
2000                                Text::new("This text has\npadding on its bottom and left."),
2001                                TextFont::from_font_size(FontSize::Px(10.)),
2002                                TextLayout::justify(text_justify),
2003                                BorderColor::all(RED),
2004                                Outline {
2005                                    width: px(2),
2006                                    color: Color::WHITE,
2007                                    ..Default::default()
2008                                },
2009                            ));
2010
2011                            builder.spawn((
2012                                Node {
2013                                    padding: px(20).top().with_left(px(20)),
2014                                    border: px(10).bottom().with_right(px(10)),
2015                                    ..default()
2016                                },
2017                                Text::new(
2018                                    "This text has\npadding on its top and left\nand a border on its bottom and right.",
2019                                ),
2020                                TextFont::from_font_size(FontSize::Px(10.)),
2021                                TextLayout::justify(text_justify),
2022                                BorderColor::all(RED),
2023                                Outline {
2024                                    width: px(2),
2025                                    color: Color::WHITE,
2026                                    ..Default::default()
2027                                },
2028                            ));
2029                        });
2030                }
2031            });
2032    }
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#937)

#### pub const fn [with\_right](#method.with_right)(self, right: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Returns the [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") with its `right` field set to the given value.

##### Example

```rust
let ui_rect = UiRect::all(Val::Px(20.0)).with_right(Val::Px(10.0));
assert_eq!(ui_rect.left, Val::Px(20.0));
assert_eq!(ui_rect.right, Val::Px(10.0));
assert_eq!(ui_rect.top, Val::Px(20.0));
assert_eq!(ui_rect.bottom, Val::Px(20.0));
```

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/3d/ssr.rs ([line 574](../../src/ssr/ssr.rs.html#574))

```rust
555fn range_controls(
556    value: f32,
557    marker: RangeValueText,
558    dec_setting: ExampleSetting,
559    inc_setting: ExampleSetting,
560) -> impl Bundle {
561    (
562        Node {
563            align_items: AlignItems::Center,
564            ..default()
565        },
566        Children::spawn((
567            Spawn(adjustment_button(dec_setting, "<", Some(true))),
568            Spawn((
569                Node {
570                    width: px(50),
571                    height: px(33),
572                    justify_content: JustifyContent::Center,
573                    align_items: AlignItems::Center,
574                    border: BUTTON_BORDER.with_left(px(0)).with_right(px(0)),
575                    ..default()
576                },
577                BackgroundColor(Color::WHITE),
578                BUTTON_BORDER_COLOR,
579                marker,
580                children![(widgets::ui_text(&format!("{:.2}", value), Color::BLACK))],
581            )),
582            Spawn(adjustment_button(inc_setting, ">", Some(false))),
583        )),
584    )
585}
586
587fn adjustment_button(
588    setting: ExampleSetting,
589    label: &str,
590    is_left_right: Option<bool>,
591) -> impl Bundle {
592    (
593        Button,
594        Node {
595            height: px(33),
596            border: if let Some(is_left) = is_left_right {
597                if is_left {
598                    BUTTON_BORDER.with_right(px(0))
599                } else {
600                    BUTTON_BORDER.with_left(px(0))
601                }
602            } else {
603                BUTTON_BORDER
604            },
605            justify_content: JustifyContent::Center,
606            align_items: AlignItems::Center,
607            padding: BUTTON_PADDING,
608            border_radius: match is_left_right {
609                Some(true) => BorderRadius::ZERO.with_left(BUTTON_BORDER_RADIUS_SIZE),
610                Some(false) => BorderRadius::ZERO.with_right(BUTTON_BORDER_RADIUS_SIZE),
611                None => BorderRadius::all(BUTTON_BORDER_RADIUS_SIZE),
612            },
613            ..default()
614        },
615        BUTTON_BORDER_COLOR,
616        BackgroundColor(Color::BLACK),
617        RadioButton,
618        WidgetClickSender(setting),
619        children![(widgets::ui_text(label, Color::WHITE), RadioButtonText)],
620    )
621}
```

Hide additional examples

examples/testbed/ui.rs ([line 1981](../../src/testbed_ui/ui.rs.html#1981))

```rust
1854    pub fn setup(mut commands: Commands) {
1855        commands.spawn((Camera2d, DespawnOnExit(super::Scene::BoxedContent)));
1856        commands
1857            .spawn((
1858                Node {
1859                    margin: auto().all(),
1860                    column_gap: px(30),
1861                    ..default()
1862                },
1863                DespawnOnExit(super::Scene::BoxedContent),
1864            ))
1865            .with_children(|builder| {
1866                for (heading, text_justify) in [
1867                    ("Left", Justify::Left),
1868                    ("Center", Justify::Center),
1869                    ("Right", Justify::Right),
1870                ] {
1871                    builder
1872                        .spawn(Node {
1873                            flex_direction: FlexDirection::Column,
1874                            align_items: AlignItems::Center,
1875                            justify_content: JustifyContent::Start,
1876                            row_gap: px(20),
1877                            ..default()
1878                        })
1879                        .with_children(|builder| {
1880                            builder.spawn((
1881                                Node::default(),
1882                                Text::new(format!("{heading} justify")),
1883                                TextFont::from_font_size(FontSize::Px(14.)),
1884                                TextLayout::justify(Justify::Center),
1885                            ));
1886
1887                            builder.spawn((
1888                                Node::default(),
1889                                Text::new("This text has\nno border or padding."),
1890                                TextFont::from_font_size(FontSize::Px(10.)),
1891                                TextLayout::justify(text_justify),
1892                                Outline {
1893                                    width: px(2),
1894                                    color: Color::WHITE,
1895                                    ..Default::default()
1896                                },
1897                            ));
1898
1899                            builder.spawn((
1900                                Node {
1901                                    border: px(10).all(),
1902                                    ..default()
1903                                },
1904                                Text::new("This text has\na border but no padding."),
1905                                TextFont::from_font_size(FontSize::Px(10.)),
1906                                TextLayout::justify(text_justify),
1907                                BorderColor::all(RED),
1908                                Outline {
1909                                    width: px(2),
1910                                    color: Color::WHITE,
1911                                    ..Default::default()
1912                                },
1913                            ));
1914
1915                            builder.spawn((
1916                                Node {
1917                                    padding: px(20).all(),
1918                                    ..default()
1919                                },
1920                                Text::new("This text has\npadding but no border."),
1921                                TextFont::from_font_size(FontSize::Px(10.)),
1922                                TextLayout::justify(text_justify),
1923                                Outline {
1924                                    width: px(2),
1925                                    color: Color::WHITE,
1926                                    ..Default::default()
1927                                },
1928                            ));
1929
1930                            builder.spawn((
1931                                Node {
1932                                    border: px(10).all(),
1933                                    padding: px(20).all(),
1934                                    ..default()
1935                                },
1936                                Text::new("This text has\nborder and padding."),
1937                                TextFont::from_font_size(FontSize::Px(10.)),
1938                                TextLayout::justify(text_justify),
1939                                BorderColor::all(RED),
1940                                Outline {
1941                                    width: px(2),
1942                                    color: Color::WHITE,
1943                                    ..Default::default()
1944                                },
1945                            ));
1946
1947                            builder.spawn((
1948                                Node {
1949                                    border: px(10).left(),
1950                                    ..default()
1951                                },
1952                                Text::new("This text has\na left border and no padding."),
1953                                TextFont::from_font_size(FontSize::Px(10.)),
1954                                TextLayout::justify(text_justify),
1955                                BorderColor::all(RED),
1956                                Outline {
1957                                    width: px(2),
1958                                    color: Color::WHITE,
1959                                    ..Default::default()
1960                                },
1961                            ));
1962
1963                            builder.spawn((
1964                                Node {
1965                                    border: px(10).right(),
1966                                    ..default()
1967                                },
1968                                Text::new("This text has\na right border and no padding."),
1969                                TextFont::from_font_size(FontSize::Px(10.)),
1970                                TextLayout::justify(text_justify),
1971                                BorderColor::all(RED),
1972                                Outline {
1973                                    width: px(2),
1974                                    color: Color::WHITE,
1975                                    ..Default::default()
1976                                },
1977                            ));
1978
1979                            builder.spawn((
1980                                Node {
1981                                    padding: px(20).top().with_right(px(20)),
1982                                    ..default()
1983                                },
1984                                Text::new("This text has\npadding on its top and right."),
1985                                TextFont::from_font_size(FontSize::Px(10.)),
1986                                TextLayout::justify(text_justify),
1987                                BorderColor::all(RED),
1988                                Outline {
1989                                    width: px(2),
1990                                    color: Color::WHITE,
1991                                    ..Default::default()
1992                                },
1993                            ));
1994
1995                            builder.spawn((
1996                                Node {
1997                                    padding: px(20).bottom().with_left(px(20)),
1998                                    ..default()
1999                                },
2000                                Text::new("This text has\npadding on its bottom and left."),
2001                                TextFont::from_font_size(FontSize::Px(10.)),
2002                                TextLayout::justify(text_justify),
2003                                BorderColor::all(RED),
2004                                Outline {
2005                                    width: px(2),
2006                                    color: Color::WHITE,
2007                                    ..Default::default()
2008                                },
2009                            ));
2010
2011                            builder.spawn((
2012                                Node {
2013                                    padding: px(20).top().with_left(px(20)),
2014                                    border: px(10).bottom().with_right(px(10)),
2015                                    ..default()
2016                                },
2017                                Text::new(
2018                                    "This text has\npadding on its top and left\nand a border on its bottom and right.",
2019                                ),
2020                                TextFont::from_font_size(FontSize::Px(10.)),
2021                                TextLayout::justify(text_justify),
2022                                BorderColor::all(RED),
2023                                Outline {
2024                                    width: px(2),
2025                                    color: Color::WHITE,
2026                                    ..Default::default()
2027                                },
2028                            ));
2029                        });
2030                }
2031            });
2032    }
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#956)

#### pub const fn [with\_top](#method.with_top)(self, top: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Returns the [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") with its `top` field set to the given value.

##### Example

```rust
let ui_rect = UiRect::all(Val::Px(20.0)).with_top(Val::Px(10.0));
assert_eq!(ui_rect.left, Val::Px(20.0));
assert_eq!(ui_rect.right, Val::Px(20.0));
assert_eq!(ui_rect.top, Val::Px(10.0));
assert_eq!(ui_rect.bottom, Val::Px(20.0));
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#975)

#### pub const fn [with\_bottom](#method.with_bottom)(self, bottom: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Returns the [`UiRect`](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") with its `bottom` field set to the given value.

##### Example

```rust
let ui_rect = UiRect::all(Val::Px(20.0)).with_bottom(Val::Px(10.0));
assert_eq!(ui_rect.left, Val::Px(20.0));
assert_eq!(ui_rect.right, Val::Px(20.0));
assert_eq!(ui_rect.top, Val::Px(20.0));
assert_eq!(ui_rect.bottom, Val::Px(10.0));
```

## Trait Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#981)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#982)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#629)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#629)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#987)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Val](../prelude/enum.Val.html "enum bevy::prelude::Val")\> for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#988)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [from\_reflect](../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [get\_represented\_type\_info](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [try\_apply](../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [reflect\_kind](../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [reflect\_ref](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [reflect\_owned](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [try\_into\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [try\_as\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [try\_as\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [into\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [as\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [as\_partial\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#626)

#### fn [reflect\_partial\_eq](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [reflect\_partial\_cmp](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#626)

#### fn [debug](../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#626)

#### fn [reflect\_clone](../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](../prelude/trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](../prelude/trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](../prelude/trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [into\_any](../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [as\_any](../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [as\_any\_mut](../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [into\_reflect](../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [as\_reflect](../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [as\_reflect\_mut](../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [set](../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#629)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#629)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [field](../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [field\_mut](../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [field\_at](../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [field\_at\_mut](../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [name\_at](../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [index\_of\_name](../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [field\_len](../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [iter\_fields](../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [to\_dynamic\_struct](../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#272)

### impl<S> [GetField](../prelude/trait.GetField.html "trait bevy::prelude::GetField") for S

where S: [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#273)

#### fn [get\_field](../prelude/trait.GetField.html#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#278)

#### fn [get\_field\_mut](../prelude/trait.GetField.html#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](../prelude/trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](../prelude/trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](../prelude/trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](../prelude/trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.path_mut)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}