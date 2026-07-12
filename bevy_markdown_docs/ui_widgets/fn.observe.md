[bevy](../index.html)::[ui\_widgets](index.html)

# Function observe 

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/observe.rs.html#77-79)

```rust
pub fn observe<E, B, M, I>(observer: I) -> AddObserver<E, B, M, I>where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
```

Adds an observer as a bundle effect.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/ui/widgets/standard\_widgets\_observers.rs ([lines 101-103](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#101-103))

```rust
85fn demo_root(asset_server: &AssetServer) -> impl Bundle {
86    (
87        Node {
88            width: percent(100),
89            height: percent(100),
90            align_items: AlignItems::Center,
91            justify_content: JustifyContent::Center,
92            display: Display::Flex,
93            flex_direction: FlexDirection::Column,
94            row_gap: px(10),
95            ..default()
96        },
97        TabGroup::default(),
98        children![
99            (
100                button(asset_server),
101                observe(|_activate: On<Activate>| {
102                    info!("Button clicked!");
103                }),
104            ),
105            (
106                slider(0.0, 100.0, 50.0),
107                observe(
108                    |value_change: On<ValueChange<f32>>,
109                     mut widget_states: ResMut<DemoWidgetStates>| {
110                        widget_states.slider_value = value_change.value;
111                    },
112                )
113            ),
114            (
115                checkbox(asset_server, "Checkbox"),
116                observe(checkbox_self_update),
117            ),
118            Text::new("Press 'D' to toggle widget disabled states"),
119        ],
120    )
121}
```

Hide additional examples

examples/ui/widgets/standard\_widgets.rs ([lines 157-159](../../src/standard_widgets/standard_widgets.rs.html#157-159))

```rust
141fn demo_root(asset_server: &AssetServer) -> impl Bundle {
142    (
143        Node {
144            width: percent(100),
145            height: percent(100),
146            align_items: AlignItems::Center,
147            justify_content: JustifyContent::Center,
148            display: Display::Flex,
149            flex_direction: FlexDirection::Column,
150            row_gap: px(10),
151            ..default()
152        },
153        TabGroup::default(),
154        children![
155            (
156                button(asset_server),
157                observe(|_activate: On<Activate>| {
158                    info!("Button clicked!");
159                }),
160            ),
161            (
162                slider(0.0, 100.0, 50.0),
163                observe(
164                    |value_change: On<ValueChange<f32>>,
165                     mut widget_states: ResMut<DemoWidgetStates>| {
166                        widget_states.slider_value = value_change.value;
167                    },
168                )
169            ),
170            (
171                checkbox(asset_server, "Checkbox"),
172                observe(checkbox_self_update)
173            ),
174            (
175                radio_group(asset_server),
176                observe(
177                    |value_change: On<ValueChange<Entity>>,
178                     mut widget_states: ResMut<DemoWidgetStates>,
179                     q_radios: Query<&DemoRadio>| {
180                        if let Ok(radio) = q_radios.get(value_change.value) {
181                            widget_states.slider_click = radio.0;
182                        }
183                    },
184                )
185            ),
186            menu_button(asset_server),
187            Text::new("Press 'D' to toggle widget disabled states"),
188        ],
189    )
190}
191
192fn button(asset_server: &AssetServer) -> impl Bundle {
193    (
194        Node {
195            width: px(150),
196            height: px(65),
197            border: UiRect::all(px(5)),
198            border_radius: BorderRadius::MAX,
199            justify_content: JustifyContent::Center,
200            align_items: AlignItems::Center,
201            ..default()
202        },
203        DemoButton,
204        Button,
205        Hovered::default(),
206        TabIndex(0),
207        BorderColor::all(Color::BLACK),
208        BackgroundColor(NORMAL_BUTTON),
209        children![(
210            Text::new("Button"),
211            TextFont {
212                font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
213                font_size: FontSize::Px(33.0),
214                ..default()
215            },
216            TextColor(Color::srgb(0.9, 0.9, 0.9)),
217            TextShadow::default(),
218        )],
219    )
220}
221
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
```

examples/ui/widgets/vertical\_slider.rs ([line 89](../../src/vertical_slider/vertical_slider.rs.html#89))

```rust
36fn setup(mut commands: Commands, assets: Res<AssetServer>) {
37    commands.spawn(Camera2d);
38
39    commands
40        .spawn((
41            Node {
42                width: percent(100),
43                height: percent(100),
44                align_items: AlignItems::Center,
45                justify_content: JustifyContent::Center,
46                display: Display::Flex,
47                flex_direction: FlexDirection::Row,
48                column_gap: px(50),
49                ..default()
50            },
51            TabGroup::default(),
52        ))
53        .with_children(|parent| {
54            // Vertical slider
55            parent
56                .spawn(Node {
57                    display: Display::Flex,
58                    flex_direction: FlexDirection::Column,
59                    align_items: AlignItems::Center,
60                    row_gap: px(10),
61                    ..default()
62                })
63                .with_children(|parent| {
64                    parent.spawn((
65                        Text::new("Vertical"),
66                        TextFont {
67                            font: assets.load("fonts/FiraSans-Bold.ttf").into(),
68                            font_size: FontSize::Px(20.0),
69                            ..default()
70                        },
71                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
72                    ));
73
74                    let label_id = parent
75                        .spawn((
76                            Text::new("50"),
77                            TextFont {
78                                font: assets.load("fonts/FiraSans-Bold.ttf").into(),
79                                font_size: FontSize::Px(24.0),
80                                ..default()
81                            },
82                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
83                        ))
84                        .id();
85
86                    parent.spawn((
87                        vertical_slider(),
88                        ValueLabel(label_id),
89                        observe(slider_self_update),
90                    ));
91                });
92
93            // Horizontal slider
94            parent
95                .spawn(Node {
96                    display: Display::Flex,
97                    flex_direction: FlexDirection::Column,
98                    align_items: AlignItems::Center,
99                    row_gap: px(10),
100                    ..default()
101                })
102                .with_children(|parent| {
103                    parent.spawn((
104                        Text::new("Horizontal"),
105                        TextFont {
106                            font: assets.load("fonts/FiraSans-Bold.ttf").into(),
107                            font_size: FontSize::Px(20.0),
108                            ..default()
109                        },
110                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
111                    ));
112
113                    let label_id = parent
114                        .spawn((
115                            Text::new("50"),
116                            TextFont {
117                                font: assets.load("fonts/FiraSans-Bold.ttf").into(),
118                                font_size: FontSize::Px(24.0),
119                                ..default()
120                            },
121                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
122                        ))
123                        .id();
124
125                    parent.spawn((
126                        horizontal_slider(),
127                        ValueLabel(label_id),
128                        observe(slider_self_update),
129                    ));
130                });
131        });
132}
```