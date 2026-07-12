[bevy](../../index.html)::[ui](../index.html)::[prelude](index.html)

# Struct BorderRadius 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2526)

```rust
pub struct BorderRadius {
    pub top_left: Val,
    pub top_right: Val,
    pub bottom_right: Val,
    pub bottom_left: Val,
}
```

Used to add rounded corners to a UI node. You can set a UI node to have uniformly rounded corners or specify different radii for each corner. If a given radius exceeds half the length of the smallest dimension between the node’s height or width, the radius will calculated as half the smallest dimension.

Elliptical nodes are not supported yet. Percentage values are based on the node’s smallest dimension, either width or height.

## Example

```rust
fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Px(100.),
            height: Val::Px(100.),
            border: UiRect::all(Val::Px(2.)),
            border_radius: BorderRadius::new(
                // top left
                Val::Px(10.),
                // top right
                Val::Px(20.),
                // bottom right
                Val::Px(30.),
                // bottom left
                Val::Px(40.),
            ),
            ..Default::default()
        },
        BackgroundColor(BLUE.into()),
    ));
}
```

[https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius](https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius)

## Fields

`top_left: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")``top_right: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")``bottom_right: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")``bottom_left: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")`

## Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2539)

### impl [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2540)

#### pub const [DEFAULT](#associatedconstant.DEFAULT): [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius") = Self::ZERO

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2543)

#### pub const [ZERO](#associatedconstant.ZERO): [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Zero curvature. All the corners will be right-angled.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2546)

#### pub const [MAX](#associatedconstant.MAX): [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Maximum curvature. The UI Node will take a capsule shape or circular if width and height are equal.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2550)

#### pub const fn [all](#method.all)(radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Set all four corners to the same curvature.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/3d/clustered\_decals.rs ([line 286](../../../src/clustered_decals/clustered_decals.rs.html#286))

```rust
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
```

Hide additional examples

examples/3d/light\_textures.rs ([line 332](../../../src/light_textures/light_textures.rs.html#332))

```rust
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
```

examples/ui/styling/box\_shadow.rs ([line 29](../../../src/box_shadow/box_shadow.rs.html#29))

```rust
20const SHAPES: &[(&str, fn(&mut Node))] = &[
21    ("1", |node| {
22        node.width = px(164);
23        node.height = px(164);
24        node.border_radius = BorderRadius::ZERO;
25    }),
26    ("2", |node| {
27        node.width = px(164);
28        node.height = px(164);
29        node.border_radius = BorderRadius::all(px(41));
30    }),
31    ("3", |node| {
32        node.width = px(164);
33        node.height = px(164);
34        node.border_radius = BorderRadius::MAX;
35    }),
36    ("4", |node| {
37        node.width = px(240);
38        node.height = px(80);
39        node.border_radius = BorderRadius::all(px(32));
40    }),
41    ("5", |node| {
42        node.width = px(80);
43        node.height = px(240);
44        node.border_radius = BorderRadius::all(px(32));
45    }),
46];
47
48#[derive(Resource, Default)]
49struct ShapeSettings {
50    index: usize,
51}
52
53#[derive(Resource, Default)]
54struct ShadowSettings {
55    x_offset: f32,
56    y_offset: f32,
57    blur: f32,
58    spread: f32,
59    count: usize,
60    samples: u32,
61}
62
63#[derive(Component)]
64struct ShadowNode;
65
66#[derive(Component, PartialEq, Clone, Copy)]
67enum SettingsButton {
68    XOffsetInc,
69    XOffsetDec,
70    YOffsetInc,
71    YOffsetDec,
72    BlurInc,
73    BlurDec,
74    SpreadInc,
75    SpreadDec,
76    CountInc,
77    CountDec,
78    ShapePrev,
79    ShapeNext,
80    Reset,
81    SamplesInc,
82    SamplesDec,
83}
84
85#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
86enum SettingType {
87    XOffset,
88    YOffset,
89    Blur,
90    Spread,
91    Count,
92    Shape,
93    Samples,
94}
95
96impl SettingType {
97    fn label(&self) -> &str {
98        match self {
99            SettingType::XOffset => "X Offset",
100            SettingType::YOffset => "Y Offset",
101            SettingType::Blur => "Blur",
102            SettingType::Spread => "Spread",
103            SettingType::Count => "Count",
104            SettingType::Shape => "Shape",
105            SettingType::Samples => "Samples",
106        }
107    }
108}
109
110#[derive(Resource, Default)]
111struct HeldButton {
112    button: Option<SettingsButton>,
113    pressed_at: Option<f64>,
114    last_repeat: Option<f64>,
115}
116
117fn main() {
118    App::new()
119        .add_plugins(DefaultPlugins)
120        .insert_resource(SHADOW_DEFAULT_SETTINGS)
121        .insert_resource(SHAPE_DEFAULT_SETTINGS)
122        .insert_resource(HeldButton::default())
123        .add_systems(Startup, setup)
124        .add_systems(
125            Update,
126            (
127                button_system,
128                button_color_system,
129                update_shape.run_if(resource_changed::<ShapeSettings>),
130                update_shadow.run_if(resource_changed::<ShadowSettings>),
131                update_shadow_samples.run_if(resource_changed::<ShadowSettings>),
132                button_repeat_system,
133            ),
134        )
135        .run();
136}
137
138// --- UI Setup ---
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
286
287// --- UI Helper Functions ---
288
289// Helper to return an input to the children! macro for a setting row
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

examples/3d/ssr.rs ([line 611](../../../src/ssr/ssr.rs.html#611))

```rust
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

examples/usage/context\_menu.rs ([line 97](../../../src/context_menu/context_menu.rs.html#97))

```rust
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

examples/ui/ui\_material.rs ([line 44](../../../src/ui_material/ui_material.rs.html#44))

```rust
20fn setup(
21    mut commands: Commands,
22    mut ui_materials: ResMut<Assets<CustomUiMaterial>>,
23    asset_server: Res<AssetServer>,
24) {
25    // Camera so we can see UI
26    commands.spawn(Camera2d);
27
28    commands
29        .spawn(Node {
30            width: percent(100),
31            height: percent(100),
32            align_items: AlignItems::Center,
33            justify_content: JustifyContent::Center,
34            ..default()
35        })
36        .with_children(|parent| {
37            let banner_scale_factor = 0.5;
38            parent.spawn((
39                Node {
40                    position_type: PositionType::Absolute,
41                    width: px(905.0 * banner_scale_factor),
42                    height: px(363.0 * banner_scale_factor),
43                    border: UiRect::all(px(20)),
44                    border_radius: BorderRadius::all(px(20)),
45                    ..default()
46                },
47                MaterialNode(ui_materials.add(CustomUiMaterial {
48                    color: LinearRgba::WHITE.to_f32_array().into(),
49                    slider: Vec4::splat(0.5),
50                    color_texture: asset_server.load("branding/banner.png"),
51                    border_color: LinearRgba::WHITE.to_f32_array().into(),
52                })),
53                // UI material nodes can have outlines and shadows like any other UI node
54                Outline {
55                    width: px(2),
56                    offset: px(100),
57                    color: DARK_BLUE.into(),
58                },
59            ));
60        });
61}
```

Additional examples can be found in:  

*   [examples/ui/widgets/standard\_widgets.rs](../../../src/standard_widgets/standard_widgets.rs.html#236)
*   [examples/ui/widgets/vertical\_slider.rs](../../../src/vertical_slider/vertical_slider.rs.html#160)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#1826)
*   [examples/camera/2d\_on\_ui.rs](../../../src/2d_on_ui/2d_on_ui.rs.html#48)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../../src/standard_widgets_observers/standard_widgets_observers.rs.html#240)
*   [examples/2d/dynamic\_mip\_generation.rs](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#339)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#291)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../../src/scrollbars/scrollbars.rs.html#115)
*   [examples/ui/render\_ui\_to\_texture.rs](../../../src/render_ui_to_texture/render_ui_to_texture.rs.html#99)
*   [examples/ui/navigation/directional\_navigation.rs](../../../src/directional_navigation/directional_navigation.rs.html#142)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#192)
*   [examples/ui/styling/gradients.rs](../../../src/gradients/gradients.rs.html#88)
*   [examples/testbed/full\_ui.rs](../../../src/testbed_full_ui/full_ui.rs.html#234)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2560)

#### pub const fn [new](#method.new)( top\_left: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val"), top\_right: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val"), bottom\_right: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val"), bottom\_left: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val"), ) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2571)

#### pub const fn [px](#method.px)( top\_left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), top\_right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom\_right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom\_left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Sets the radii to logical pixel values.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/testbed/ui.rs ([lines 807-812](../../../src/testbed_ui/ui.rs.html#807-812))

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

Hide additional examples

examples/ui/styling/borders.rs ([lines 177-182](../../../src/borders/borders.rs.html#177-182))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2582-2587)

#### pub const fn [percent](#method.percent)( top\_left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), top\_right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom\_right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom\_left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Sets the radii to percentage values.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2599)

#### pub const fn [top\_left](#method.top_left)(radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Sets the radius for the top left corner. Remaining corners will be right-angled.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1830](../../../src/testbed_ui/ui.rs.html#1830))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2609)

#### pub const fn [top\_right](#method.top_right)(radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Sets the radius for the top right corner. Remaining corners will be right-angled.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1828](../../../src/testbed_ui/ui.rs.html#1828))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2619)

#### pub const fn [bottom\_right](#method.bottom_right)(radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Sets the radius for the bottom right corner. Remaining corners will be right-angled.

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 860](../../../src/testbed_ui/ui.rs.html#860))

```rust
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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2629)

#### pub const fn [bottom\_left](#method.bottom_left)(radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Sets the radius for the bottom left corner. Remaining corners will be right-angled.

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1824](../../../src/testbed_ui/ui.rs.html#1824))

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

examples/3d/solari.rs ([line 185](../../../src/solari/solari.rs.html#185))

```rust
77fn setup_pica_pica(
78    mut commands: Commands,
79    asset_server: Res<AssetServer>,
80    args: Res<Args>,
81    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))] dlss_rr_supported: Option<
82        Res<DlssRayReconstructionSupported>,
83    >,
84) {
85    commands
86        .spawn((
87            WorldAssetRoot(
88                asset_server.load(
89                    GltfAssetLabel::Scene(0)
90                        .from_asset("https://github.com/bevyengine/bevy_asset_files/raw/2a5950295a8b6d9d051d59c0df69e87abcda58c3/pica_pica/mini_diorama_01.glb")
91                ),
92            ),
93            Transform::from_scale(Vec3::splat(10.0)),
94        ))
95        .observe(add_raytracing_meshes_on_scene_load);
96
97    commands
98        .spawn((
99            WorldAssetRoot(asset_server.load(
100                GltfAssetLabel::Scene(0).from_asset("https://github.com/bevyengine/bevy_asset_files/raw/2a5950295a8b6d9d051d59c0df69e87abcda58c3/pica_pica/robot_01.glb")
101            )),
102            Transform::from_scale(Vec3::splat(2.0))
103                .with_translation(Vec3::new(-2.0, 0.05, -2.1))
104                .with_rotation(Quat::from_rotation_y(PI / 2.0)),
105            PatrolPath {
106                path: vec![
107                    (Vec3::new(-2.0, 0.05, -2.1), Quat::from_rotation_y(PI / 2.0)),
108                    (Vec3::new(2.2, 0.05, -2.1), Quat::from_rotation_y(0.0)),
109                    (
110                        Vec3::new(2.2, 0.05, 2.1),
111                        Quat::from_rotation_y(3.0 * PI / 2.0),
112                    ),
113                    (Vec3::new(-2.0, 0.05, 2.1), Quat::from_rotation_y(PI)),
114                ],
115                i: 0,
116            },
117        ))
118        .observe(add_raytracing_meshes_on_scene_load);
119
120    commands.spawn((
121        DirectionalLight {
122            illuminance: light_consts::lux::FULL_DAYLIGHT,
123            shadow_maps_enabled: false, // Solari replaces shadow mapping
124            ..default()
125        },
126        Transform::from_rotation(Quat::from_xyzw(
127            -0.13334629,
128            -0.86597735,
129            -0.3586996,
130            0.3219264,
131        )),
132    ));
133
134    let mut camera = commands.spawn((
135        Camera3d::default(),
136        Camera {
137            clear_color: ClearColorConfig::Custom(Color::BLACK),
138            ..default()
139        },
140        FreeCamera {
141            walk_speed: 3.0,
142            run_speed: 10.0,
143            ..Default::default()
144        },
145        Transform::from_translation(Vec3::new(0.219417, 2.5764852, 6.9718704)).with_rotation(
146            Quat::from_xyzw(-0.1466768, 0.013738206, 0.002037309, 0.989087),
147        ),
148        // Msaa::Off and CameraMainTextureUsages with STORAGE_BINDING are required for Solari
149        CameraMainTextureUsages::default().with(TextureUsages::STORAGE_BINDING),
150        Msaa::Off,
151    ));
152
153    if args.pathtracer == Some(true) {
154        camera.insert(Pathtracer::default());
155    } else {
156        camera.insert(SolariLighting::default());
157    }
158
159    // Using DLSS Ray Reconstruction for denoising (and cheaper rendering via upscaling) is _highly_ recommended when using Solari
160    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))]
161    if dlss_rr_supported.is_some() {
162        camera.insert(Dlss::<DlssRayReconstructionFeature> {
163            perf_quality_mode: Default::default(),
164            reset: Default::default(),
165            _phantom_data: Default::default(),
166        });
167    }
168
169    commands.spawn((
170        ControlText,
171        Text::default(),
172        Node {
173            position_type: PositionType::Absolute,
174            bottom: px(12.0),
175            left: px(12.0),
176            ..default()
177        },
178    ));
179
180    commands.spawn((
181        Node {
182            position_type: PositionType::Absolute,
183            right: px(0.0),
184            padding: px(4.0).all(),
185            border_radius: BorderRadius::bottom_left(px(4.0)),
186            ..default()
187        },
188        BackgroundColor(Color::srgba(0.10, 0.10, 0.10, 0.8)),
189        children![(
190            PerformanceText,
191            Text::default(),
192            TextFont {
193                font_size: FontSize::Px(8.0),
194                ..default()
195            },
196        )],
197    ));
198}
199
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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2639)

#### pub const fn [left](#method.left)(radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Sets the radii for the top left and bottom left corners. Remaining corners will be right-angled.

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1825](../../../src/testbed_ui/ui.rs.html#1825))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2650)

#### pub const fn [right](#method.right)(radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Sets the radii for the top right and bottom right corners. Remaining corners will be right-angled.

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1827](../../../src/testbed_ui/ui.rs.html#1827))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2661)

#### pub const fn [top](#method.top)(radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Sets the radii for the top left and top right corners. Remaining corners will be right-angled.

##### [Examples found in repository](#scraped-examples-8)[?](../../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1823](../../../src/testbed_ui/ui.rs.html#1823))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2672)

#### pub const fn [bottom](#method.bottom)(radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Sets the radii for the bottom left and bottom right corners. Remaining corners will be right-angled.

##### [Examples found in repository](#scraped-examples-9)[?](../../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1829](../../../src/testbed_ui/ui.rs.html#1829))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2682)

#### pub const fn [with\_top\_left](#method.with_top_left)(self, radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Returns the [`BorderRadius`](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius") with its `top_left` field set to the given value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2689)

#### pub const fn [with\_top\_right](#method.with_top_right)(self, radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Returns the [`BorderRadius`](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius") with its `top_right` field set to the given value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2696)

#### pub const fn [with\_bottom\_right](#method.with_bottom_right)(self, radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Returns the [`BorderRadius`](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius") with its `bottom_right` field set to the given value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2703)

#### pub const fn [with\_bottom\_left](#method.with_bottom_left)(self, radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Returns the [`BorderRadius`](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius") with its `bottom_left` field set to the given value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2710)

#### pub const fn [with\_left](#method.with_left)(self, radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Returns the [`BorderRadius`](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius") with its `top_left` and `bottom_left` fields set to the given value.

##### [Examples found in repository](#scraped-examples-10)[?](../../../scrape-examples-help.html)

examples/3d/ssr.rs ([line 609](../../../src/ssr/ssr.rs.html#609))

```rust
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

examples/3d/../helpers/widgets.rs ([lines 87-91](../../../src/clustered_decal_maps/helpers/widgets.rs.html#87-91))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2718)

#### pub const fn [with\_right](#method.with_right)(self, radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Returns the [`BorderRadius`](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius") with its `top_right` and `bottom_right` fields set to the given value.

##### [Examples found in repository](#scraped-examples-11)[?](../../../scrape-examples-help.html)

examples/3d/ssr.rs ([line 610](../../../src/ssr/ssr.rs.html#610))

```rust
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

examples/3d/../helpers/widgets.rs ([lines 92-96](../../../src/clustered_decal_maps/helpers/widgets.rs.html#92-96))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2726)

#### pub const fn [with\_top](#method.with_top)(self, radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Returns the [`BorderRadius`](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius") with its `top_left` and `top_right` fields set to the given value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2734)

#### pub const fn [with\_bottom](#method.with_bottom)(self, radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Returns the [`BorderRadius`](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius") with its `bottom_left` and `bottom_right` fields set to the given value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2742-2747)

#### pub const fn [resolve\_single\_corner](#method.resolve_single_corner)( radius: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val"), scale\_factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), min\_length: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), viewport\_size: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), ) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Resolve the border radius for a single corner from the given context values. Returns the radius of the corner in physical pixels.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2757-2762)

#### pub const fn [resolve](#method.resolve)( &self, scale\_factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), node\_size: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), viewport\_size: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), ) -> [ResolvedBorderRadius](../../prelude/struct.ResolvedBorderRadius.html "struct bevy::prelude::ResolvedBorderRadius")

Resolve the border radii for the corners from the given context values. Returns the radii of the each corner in physical pixels.

## Trait Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2533)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2534)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2523)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2523)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2793)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")\> for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2794)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### type [This](../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

The type to convert into. [Read more](../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [from\_arg](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)( arg: [Arg](../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius") as [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [from\_reflect](../../prelude/trait.FromReflect.html#tymethod.from_reflect)( reflect: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [GetOwnership](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [ownership](../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [get\_type\_registration](../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [register\_type\_dependencies](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [IntoReturn](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [into\_return](../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius"): 'into\_return,

Converts [`Self`](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [get\_represented\_type\_info](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [try\_apply](../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [reflect\_kind](../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [reflect\_ref](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [reflect\_owned](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")\>) -> [ReflectOwned](../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [try\_into\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [try\_as\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [try\_as\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [into\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [as\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [as\_partial\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2520)

#### fn [reflect\_partial\_eq](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [reflect\_partial\_cmp](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2520)

#### fn [debug](../../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2520)

#### fn [reflect\_clone](../../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](../../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](../../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](../../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](../../prelude/trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](../../prelude/trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](../../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [into\_any](../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [as\_any](../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [as\_any\_mut](../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [into\_reflect](../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [as\_reflect](../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [as\_reflect\_mut](../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [set](../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2523)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2523)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [field](../../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [field\_mut](../../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [field\_at](../../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [field\_at\_mut](../../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [name\_at](../../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [index\_of\_name](../../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [field\_len](../../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [iter\_fields](../../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [to\_dynamic\_struct](../../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [type\_path](../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [short\_type\_path](../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [type\_ident](../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [crate\_name](../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [module\_path](../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### fn [type\_info](../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

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

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](../../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](../../prelude/trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](../../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](../../prelude/trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](../../prelude/trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

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

### impl<T> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#272)

### impl<S> [GetField](../../prelude/trait.GetField.html "trait bevy::prelude::GetField") for S

where S: [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#273)

#### fn [get\_field](../../prelude/trait.GetField.html#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#278)

#### fn [get\_field\_mut](../../prelude/trait.GetField.html#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](../../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](../../prelude/trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](../../prelude/trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](../../prelude/trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](../../prelude/trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.path_mut)

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

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

### impl<T> [IntoResult](../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

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

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#method.clone_into)

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

### impl<T> [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}