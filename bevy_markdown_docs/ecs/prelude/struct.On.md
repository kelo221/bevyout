[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Struct On 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#38)

```rust
pub struct On<'w, 't, E, B = ()>where
    E: Event,
    B: Bundle,{ /* private fields */ }
```

A [system parameter](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") used by an observer to process events. See [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") and [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") for examples.

`On` contains the triggered [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") data for a given run of an `Observer`. It also provides access to the [`Trigger`](../event/trait.Trigger.html "trait bevy::ecs::event::Trigger"), which for things like [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") with a [`PropagateEntityTrigger`](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger"), includes control over event propagation.

The generic `B: Bundle` is used to further specialize the events that this observer is interested in. The entity involved _does not_ have to have these components, but the observer will only be triggered if the event matches the components in `B`.

This is used to avoid providing a generic argument in your event, as is done for [`Add`](../../prelude/struct.Add.html "struct bevy::prelude::Add") and the other lifecycle events.

Providing multiple components in this bundle will cause this event to be triggered by any matching component in the bundle, [rather than requiring all of them to be present](https://github.com/bevyengine/bevy/issues/15325).

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#49)

### impl<'w, 't, E, B> [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

where E: [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#51-56)

#### pub fn [new](#method.new)( event: [&'w mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html), observer: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), trigger: &'w mut <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'t>, trigger\_context: &'w [TriggerContext](../observer/struct.TriggerContext.html "struct bevy::ecs::observer::TriggerContext"), ) -> [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

Creates a new instance of [`On`](../../prelude/struct.On.html "struct bevy::prelude::On") for the given triggered event.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#67)

#### pub fn [event\_key](#method.event_key)(&self) -> [EventKey](../event/struct.EventKey.html "struct bevy::ecs::event::EventKey")

Returns the event type of this [`On`](../../prelude/struct.On.html "struct bevy::prelude::On") instance.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#72)

#### pub fn [event](#method.event)(&self) -> [&E](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Returns a reference to the triggered event.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ui/images/image\_node\_resizing.rs ([line 248](../../../src/image_node_resizing/image_node_resizing.rs.html#248))

```rust
246fn on_trigger_image_group(event: On<ImageGroupResize>, query: Query<&mut Node, With<ImageGroup>>) {
247    for mut node in query {
248        match event.event() {
249            ImageGroupResize::HeightGrow => {
250                if let Val::Percent(val) = &mut node.height {
251                    *val = (*val + MIN_RESIZE_VAL).min(IMAGE_GROUP_BOX_MAX_HEIGHT);
252                }
253            }
254            ImageGroupResize::HeightShrink => {
255                if let Val::Percent(val) = &mut node.height {
256                    *val = (*val - MIN_RESIZE_VAL).max(IMAGE_GROUP_BOX_MIN_HEIGHT);
257                }
258            }
259            ImageGroupResize::WidthGrow => {
260                if let Val::Percent(val) = &mut node.width {
261                    *val = (*val + MIN_RESIZE_VAL).min(IMAGE_GROUP_BOX_MAX_WIDTH);
262                }
263            }
264            ImageGroupResize::WidthShrink => {
265                if let Val::Percent(val) = &mut node.width {
266                    *val = (*val - MIN_RESIZE_VAL).max(IMAGE_GROUP_BOX_MIN_WIDTH);
267                }
268            }
269        }
270    }
271}
```

Hide additional examples

examples/ui/scroll\_and\_overflow/scroll.rs ([line 177](../../../src/scroll/scroll.rs.html#177))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#77)

#### pub fn [event\_mut](#method.event_mut)(&mut self) -> [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Returns a mutable reference to the triggered event.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#82)

#### pub fn [event\_ptr](#method.event_ptr)(&self) -> [Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'\_>

Returns a pointer to the triggered event.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#87)

#### pub fn [trigger](#method.trigger)(&self) -> &<E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'t>

Returns the [`Trigger`](../event/trait.Trigger.html "trait bevy::ecs::event::Trigger") context for this event.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/animation/animated\_mesh\_events.rs ([line 52](../../../src/animated_mesh_events/animated_mesh_events.rs.html#52))

```rust
45fn observe_on_step(
46    step: On<Step>,
47    particle: Res<ParticleAssets>,
48    mut commands: Commands,
49    transforms: Query<&GlobalTransform>,
50    mut seeded_rng: ResMut<SeededRng>,
51) -> Result {
52    let translation = transforms.get(step.trigger().target)?.translation();
53    // Spawn a bunch of particles.
54    for _ in 0..14 {
55        let horizontal = seeded_rng.0.random::<Dir2>() * seeded_rng.0.random_range(8.0..12.0);
56        let vertical = seeded_rng.0.random_range(0.0..4.0);
57        let size = seeded_rng.0.random_range(0.2..1.0);
58
59        commands.spawn((
60            Particle {
61                lifetime_timer: Timer::from_seconds(
62                    seeded_rng.0.random_range(0.2..0.6),
63                    TimerMode::Once,
64                ),
65                size,
66                velocity: Vec3::new(horizontal.x, vertical, horizontal.y) * 10.0,
67            },
68            Mesh3d(particle.mesh.clone()),
69            MeshMaterial3d(particle.material.clone()),
70            Transform {
71                translation,
72                scale: Vec3::splat(size),
73                ..Default::default()
74            },
75        ));
76    }
77    Ok(())
78}
```

Hide additional examples

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 337](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#337))

```rust
330fn observe_on_step(
331    step: On<Step>,
332    particle: Res<ParticleAssets>,
333    mut commands: Commands,
334    transforms: Query<&GlobalTransform>,
335    mut seeded_rng: ResMut<SeededRng>,
336) -> Result {
337    let translation = transforms.get(step.trigger().target)?.translation();
338    // Spawn a bunch of particles.
339    for _ in 0..14 {
340        let horizontal = seeded_rng.0.random::<Dir2>() * seeded_rng.0.random_range(8.0..12.0);
341        let vertical = seeded_rng.0.random_range(0.0..4.0);
342        let size = seeded_rng.0.random_range(0.2..1.0);
343
344        commands.spawn((
345            Particle {
346                lifetime_timer: Timer::from_seconds(
347                    seeded_rng.0.random_range(0.2..0.6),
348                    TimerMode::Once,
349                ),
350                size,
351                velocity: Vec3::new(horizontal.x, vertical, horizontal.y) * 10.0,
352            },
353            Mesh3d(particle.mesh.clone()),
354            MeshMaterial3d(particle.material.clone()),
355            Transform {
356                translation,
357                scale: Vec3::splat(size),
358                ..Default::default()
359            },
360        ));
361    }
362    Ok(())
363}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#92)

#### pub fn [trigger\_mut](#method.trigger_mut)(&mut self) -> &mut <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'t>

Returns the mutable [`Trigger`](../event/trait.Trigger.html "trait bevy::ecs::event::Trigger") context for this event.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#118)

#### pub fn [observer](#method.observer)(&self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Returns the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") of the [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") of the triggered event. This allows you to despawn the observer, ceasing observation.

##### Examples

```rust
#[derive(EntityEvent)]  
struct AssertEvent {
    entity: Entity,
}

fn assert_observer(event: On<AssertEvent>) {  
    assert_eq!(event.observer(), event.entity);  
}  

let mut world = World::new();  
let entity = world.spawn(Observer::new(assert_observer)).id();  

world.trigger(AssertEvent { entity });
```  

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#123)

#### pub fn [caller](#method.caller)(&self) -> [MaybeLocation](../change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")

Returns the source code location that triggered this observer, if the `track_location` cargo feature is enabled.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#128-135)

### impl<'w, 't, const AUTO\_PROPAGATE: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), E, B, T> [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

where E: [EntityEvent](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent")<Trigger<'a> = [PropagateEntityTrigger](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")<AUTO\_PROPAGATE, E, T>> + for<'a> [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), T: [Traversal](../traversal/trait.Traversal.html "trait bevy::ecs::traversal::Traversal")<E>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#139)

#### pub fn [original\_event\_target](#method.original_event_target)(&self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Returns the original [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") that this [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") targeted via [`EntityEvent::event_target`](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") when it was _first_ triggered, prior to any propagation logic.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/usage/context\_menu.rs ([line 46](../../../src/context_menu/context_menu.rs.html#46))

```rust
40fn text_color_on_hover<T: Debug + Clone + Reflect>(
41    color: Color,
42) -> impl FnMut(On<Pointer<T>>, Query<&mut TextColor>, Query<&Children>) {
43    move |mut event: On<Pointer<T>>,
44          mut text_color: Query<&mut TextColor>,
45          children: Query<&Children>| {
46        let Ok(children) = children.get(event.original_event_target()) else {
47            return;
48        };
49        event.propagate(false);
50
51        // find the text among children and change its color
52        for child in children.iter() {
53            if let Ok(mut col) = text_color.get_mut(child) {
54                col.0 = color;
55            }
56        }
57    }
58}
59
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

examples/ui/text/multiline\_text\_input.rs ([line 168](../../../src/multiline_text_input/multiline_text_input.rs.html#168))

```rust
28fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
29    commands.spawn(Camera2d);
30
31    commands
32        .spawn(Node {
33            width: percent(100.),
34            height: percent(100.),
35            justify_content: JustifyContent::Center,
36            align_items: AlignItems::Center,
37            ..default()
38        })
39        .with_children(|parent| {
40            parent
41                .spawn((
42                    Node {
43                        flex_direction: FlexDirection::Column,
44                        align_items: AlignItems::End,
45                        row_gap: px(10.),
46                        ..default()
47                    },
48                    TabGroup::default(),
49                ))
50                .with_children(|parent| {
51                    parent
52                        .spawn((
53                            Node {
54                                width: px(450.),
55                                border: px(2.).all(),
56                                padding: px(8.).all(),
57                                ..default()
58                            },
59                            EditableText {
60                                visible_lines: Some(8.),
61                                allow_newlines: true,
62                                ..default()
63                            },
64                            TextLayout {
65                                linebreak: LineBreak::WordOrCharacter,
66                                ..default()
67                            },
68                            TextCursorStyle {
69                                color: Color::WHITE,
70                                selected_text_color: Some(Color::BLACK),
71                                ..default()
72                            },
73                            TextFont {
74                                font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
75                                font_size: FontSize::Px(30.),
76                                ..default()
77                            },
78                            BackgroundColor(DARK_SLATE_GRAY.into()),
79                            BorderColor::all(SLATE_300),
80                            MultilineInput,
81                            TabIndex(0),
82                            AutoFocus,
83                        ))
84                        .observe(
85                            |on: On<FocusedInput<KeyboardInput>>,
86                             keys: Res<ButtonInput<Key>>,
87                             input_query: Query<&EditableText, With<MultilineInput>>| {
88                                if !(on.input.state.is_pressed()
89                                    && on.input.logical_key == Key::Enter
90                                    && keys.pressed(Key::Control))
91                                {
92                                    return;
93                                }
94                                let Ok(input) = input_query.get(on.focused_entity) else {
95                                    return;
96                                };
97
98                                let mut output = String::new();
99                                output.reserve(input.value().into_iter().map(str::len).sum());
100                                for sub_str in input.value() {
101                                    output.push_str(sub_str);
102                                }
103
104                                info!("{output}"                                    );
105                            },
106                        );
107
108                    parent
109                        .spawn((
110                            Node {
111                                flex_direction: FlexDirection::Row,
112                                column_gap: px(10.),
113                                ..default()
114                            },
115                            children![
116                                (
117                                    Text::new("visible lines:"),
118                                    TextFont {
119                                        font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
120                                        font_size: FontSize::Px(30.),
121                                        ..default()
122                                    },
123                                ),
124                                (
125                                    Node {
126                                        width: px(100.),
127                                        border: px(2.).all(),
128                                        ..default()
129                                    },
130                                    TextFont {
131                                        font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
132                                        font_size: FontSize::Px(30.),
133                                        ..default()
134                                    },
135                                    TextLayout {
136                                        justify: Justify::End,
137                                        ..default()
138                                    },
139                                    BackgroundColor(DARK_SLATE_GRAY.into()),
140                                    BorderColor::all(SLATE_300),
141                                    EditableText::new("8"),
142                                    EditableTextFilter::new(|c| c.is_ascii_digit() || c == '.'),
143                                    TextCursorStyle {
144                                        color: Color::WHITE,
145                                        selected_text_color: Some(Color::BLACK),
146                                        unfocused_selection_color: Color::NONE,
147                                        ..default()
148                                    },
149                                    SelectAllOnFocus,
150                                    VisibleLinesInput,
151                                    TabIndex(1),
152                                )
153                            ],
154                        ))
155                        .observe(
156                            |on: On<FocusedInput<KeyboardInput>>,
157                             mut query_set: ParamSet<(
158                                Query<&EditableText, With<VisibleLinesInput>>,
159                                Query<&mut EditableText, With<MultilineInput>>,
160                            )>| {
161                                if !(on.input.state.is_pressed()
162                                    && on.input.logical_key == Key::Enter)
163                                {
164                                    return;
165                                }
166
167                                let visible_lines_query = query_set.p0();
168                                let Ok(input) = visible_lines_query.get(on.original_event_target())
169                                else {
170                                    return;
171                                };
172
173                                let mut output = String::new();
174                                output.reserve(input.value().into_iter().map(str::len).sum());
175                                for sub_str in input.value() {
176                                    output.push_str(sub_str);
177                                }
178
179                                let Ok(lines) = output.parse::<f32>() else {
180                                    return;
181                                };
182
183                                let mut multiline_query = query_set.p1();
184                                let Ok(mut multiline_input) = multiline_query.single_mut() else {
185                                    return;
186                                };
187
188                                multiline_input.visible_lines = Some(lines.clamp(1., 10.));
189                            },
190                        );
191
192                    parent
193                        .spawn((
194                            Node {
195                                flex_direction: FlexDirection::Row,
196                                column_gap: px(10.),
197                                ..default()
198                            },
199                            children![
200                                (
201                                    Text::new("font size:"),
202                                    TextFont {
203                                        font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
204                                        font_size: FontSize::Px(30.),
205                                        ..default()
206                                    },
207                                ),
208                                (
209                                    Node {
210                                        width: px(100.),
211                                        border: px(2.).all(),
212                                        ..default()
213                                    },
214                                    TextFont {
215                                        font: asset_server.load("fonts/FiraMono-Medium.ttf").into(),
216                                        font_size: FontSize::Px(30.),
217                                        ..default()
218                                    },
219                                    TextLayout {
220                                        justify: Justify::End,
221                                        ..default()
222                                    },
223                                    BackgroundColor(DARK_SLATE_GRAY.into()),
224                                    BorderColor::all(SLATE_300),
225                                    EditableText::new("30"),
226                                    EditableTextFilter::new(|c| c.is_ascii_digit()),
227                                    TextCursorStyle {
228                                        color: Color::WHITE,
229                                        selected_text_color: Some(Color::BLACK),
230                                        unfocused_selection_color: Color::NONE,
231                                        ..default()
232                                    },
233                                    SelectAllOnFocus,
234                                    FontSizeInput,
235                                    TabIndex(2),
236                                )
237                            ],
238                        ))
239                        .observe(
240                            |on: On<FocusedInput<KeyboardInput>>,
241                             font_size_input_query: Query<&EditableText, With<FontSizeInput>>,
242                             mut multiline_input_font: Single<
243                                &mut TextFont,
244                                With<MultilineInput>,
245                            >| {
246                                if !(on.input.state.is_pressed()
247                                    && on.input.logical_key == Key::Enter)
248                                {
249                                    return;
250                                }
251
252                                let Ok(input) =
253                                    font_size_input_query.get(on.original_event_target())
254                                else {
255                                    return;
256                                };
257
258                                let mut output = String::new();
259                                output.reserve(input.value().into_iter().map(str::len).sum());
260                                for sub_str in input.value() {
261                                    output.push_str(sub_str);
262                                }
263
264                                let Ok(font_size) = output.parse::<f32>() else {
265                                    return;
266                                };
267
268                                multiline_input_font.font_size =
269                                    FontSize::Px(font_size.clamp(5., 50.));
270                            },
271                        );
272                });
273        });
274}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#156)

#### pub fn [propagate](#method.propagate)(&mut self, should\_propagate: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

Enables or disables event propagation, allowing the same event to trigger observers on a chain of different entities.

The path an [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") will propagate along is specified by the [`Traversal`](../traversal/trait.Traversal.html "trait bevy::ecs::traversal::Traversal") component defined in [`PropagateEntityTrigger`](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger").

[`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") does not propagate by default. To enable propagation, you must:

*   Enable propagation in [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") using `#[entity_event(propagate)]`. See [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for details.
*   Either call `propagate(true)` in the first observer or in the [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") derive add `#[entity_event(auto_propagate)]`.

You can prevent an event from propagating further using `propagate(false)`. This will prevent the event from triggering on the next [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") in the [`Traversal`](../traversal/trait.Traversal.html "trait bevy::ecs::traversal::Traversal"), but note that all remaining observers for the _current_ entity will still run.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/asset/asset\_saving\_with\_subassets.rs ([line 354](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#354))

```rust
347fn stop_propagate_on_clicked_box(mut event: On<Pointer<Press>>, boxes: Query<(), With<Box>>) {
348    if event.button != PointerButton::Primary {
349        return;
350    }
351    if !boxes.contains(event.entity) {
352        return;
353    }
354    event.propagate(false);
355}
```

Hide additional examples

examples/ui/navigation/directional\_navigation.rs ([line 91](../../../src/directional_navigation/directional_navigation.rs.html#91))

```rust
83fn universal_button_click_behavior(
84    mut click: On<Pointer<Click>>,
85    mut button_query: Query<(&mut BackgroundColor, &mut ResetTimer)>,
86) {
87    let button_entity = click.entity;
88    if let Ok((mut color, mut reset_timer)) = button_query.get_mut(button_entity) {
89        color.0 = PRESSED_BUTTON.into();
90        reset_timer.0 = Timer::from_seconds(0.3, TimerMode::Once);
91        click.propagate(false);
92    }
93}
```

examples/ui/navigation/directional\_navigation\_overrides.rs ([line 129](../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#129))

```rust
121fn universal_button_click_behavior(
122    mut click: On<Pointer<Click>>,
123    mut button_query: Query<(&mut BackgroundColor, &Page, &mut ResetTimer)>,
124) {
125    let button_entity = click.entity;
126    if let Ok((mut color, page, mut reset_timer)) = button_query.get_mut(button_entity) {
127        color.0 = PRESSED_BUTTON_COLORS[page.0].into();
128        reset_timer.0 = Timer::from_seconds(0.3, TimerMode::Once);
129        click.propagate(false);
130    }
131}
```

examples/usage/context\_menu.rs ([line 49](../../../src/context_menu/context_menu.rs.html#49))

```rust
40fn text_color_on_hover<T: Debug + Clone + Reflect>(
41    color: Color,
42) -> impl FnMut(On<Pointer<T>>, Query<&mut TextColor>, Query<&Children>) {
43    move |mut event: On<Pointer<T>>,
44          mut text_color: Query<&mut TextColor>,
45          children: Query<&Children>| {
46        let Ok(children) = children.get(event.original_event_target()) else {
47            return;
48        };
49        event.propagate(false);
50
51        // find the text among children and change its color
52        for child in children.iter() {
53            if let Ok(mut col) = text_color.get_mut(child) {
54                col.0 = color;
55            }
56        }
57    }
58}
59
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
124
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

examples/ecs/observer\_propagation.rs ([line 97](../../../src/observer_propagation/observer_propagation.rs.html#97))

```rust
86fn block_attack(mut attack: On<Attack>, armor: Query<(&Armor, &Name)>) {
87    let (armor, name) = armor.get(attack.entity).unwrap();
88    let damage = attack.damage.saturating_sub(**armor);
89    if damage > 0 {
90        info!("🩸 {} damage passed through {}", damage, name);
91        // The attack isn't stopped by the armor. We reduce the damage of the attack, and allow
92        // it to continue on to the goblin.
93        attack.damage = damage;
94    } else {
95        info!("🛡️  {} damage blocked by {}", attack.damage, name);
96        // Armor stopped the attack, the event stops here.
97        attack.propagate(false);
98        info!("(propagation halted early)\n");
99    }
100}
```

examples/ui/scroll\_and\_overflow/scroll.rs ([line 104](../../../src/scroll/scroll.rs.html#104))

```rust
61fn on_scroll_handler(
62    mut scroll: On<Scroll>,
63    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
64) {
65    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
66        return;
67    };
68
69    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();
70
71    let delta = &mut scroll.delta;
72    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
73        // Is this node already scrolled all the way in the direction of the scroll?
74        let max = if delta.x > 0. {
75            scroll_position.x >= max_offset.x
76        } else {
77            scroll_position.x <= 0.
78        };
79
80        if !max {
81            scroll_position.x += delta.x;
82            // Consume the X portion of the scroll delta.
83            delta.x = 0.;
84        }
85    }
86
87    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
88        // Is this node already scrolled all the way in the direction of the scroll?
89        let max = if delta.y > 0. {
90            scroll_position.y >= max_offset.y
91        } else {
92            scroll_position.y <= 0.
93        };
94
95        if !max {
96            scroll_position.y += delta.y;
97            // Consume the Y portion of the scroll delta.
98            delta.y = 0.;
99        }
100    }
101
102    // Stop propagating when the delta is fully consumed.
103    if *delta == Vec2::ZERO {
104        scroll.propagate(false);
105    }
106}
```

Additional examples can be found in:  

*   [examples/picking/dragdrop\_picking.rs](../../../src/dragdrop_picking/dragdrop_picking.rs.html#70)
*   [examples/ui/widgets/tab\_navigation.rs](../../../src/tab_navigation/tab_navigation.rs.html#85)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#163)

#### pub fn [get\_propagate](#method.get_propagate)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns the value of the flag that controls event propagation. See [`propagate`](../../prelude/struct.On.html#method.propagate "method bevy::prelude::On::propagate") for more information.

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#168)

### impl<'w, 't, E, B> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

where E: for<'a> [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"), <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'a>: for<'a> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#169)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#178)

### impl<'w, 't, E, B> [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

where E: [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#179)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = E

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#181)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#186)

### impl<'w, 't, E, B> [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

where E: [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/system_param.rs.html#187)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#250)

### impl<E, B> [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'\_, '\_, E, B>

where E: [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Used for [`ObserverSystem`](../system/trait.ObserverSystem.html "trait bevy::ecs::system::ObserverSystem")s.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#255)

#### type [Param](../../prelude/trait.SystemInput.html#associatedtype.Param)<'i> = [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'i, 'i, E, B>

The wrapper input type that is defined as the first argument to [`FunctionSystem`](../system/struct.FunctionSystem.html "struct bevy::ecs::system::FunctionSystem")s.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#256)

#### type [Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner)<'i> = [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'i, 'i, E, B>

The inner input type that is passed to functions that run systems, such as [`System::run`](../../prelude/trait.System.html#method.run "method bevy::prelude::System::run").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#258)

#### fn [wrap](../../prelude/trait.SystemInput.html#tymethod.wrap)( this: <[On](../../prelude/struct.On.html "struct bevy::prelude::On")<'\_, '\_, E, B> as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, ) -> <[On](../../prelude/struct.On.html "struct bevy::prelude::On")<'\_, '\_, E, B> as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Param](../../prelude/trait.SystemInput.html#associatedtype.Param "type bevy::prelude::SystemInput::Param")<'\_>

Converts a [`SystemInput::Inner`](../../prelude/trait.SystemInput.html#associatedtype.Inner "associated type bevy::prelude::SystemInput::Inner") into a [`SystemInput::Param`](../../prelude/trait.SystemInput.html#associatedtype.Param "associated type bevy::prelude::SystemInput::Param").

## Auto Trait Implementations

### impl<'w, 't, E, B = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

### impl<'w, 't, E, B> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

### impl<'w, 't, E, B> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

where E: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"), <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'t>: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"), B: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

### impl<'w, 't, E, B> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

where <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'t>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

### impl<'w, 't, E, B> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

where <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'t>: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

### impl<'w, 't, E, B> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

where B: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

### impl<'w, 't, E, B> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'w, 't, E, B>

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

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#97)

### impl<R> [CryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.CryptoRng.html "trait rand_core::CryptoRng") for R

where R: [TryCryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryCryptoRng.html "trait rand_core::TryCryptoRng")<Error = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#206)

### impl<T> [CryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.CryptoRng.html "trait rand_core::CryptoRng") for T

where T: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut"), <T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [CryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.CryptoRng.html "trait rand_core::CryptoRng"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#355-358)

### impl<T, C, D> [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T> for D

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), D: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = C>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#360)

#### fn [domain](../../prelude/trait.Curve.html#tymethod.domain)(&self) -> [Interval](../../prelude/struct.Interval.html "struct bevy::prelude::Interval")

The interval over which this curve is parametrized. [Read more](../../prelude/trait.Curve.html#tymethod.domain)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#364)

#### fn [sample\_unchecked](../../prelude/trait.Curve.html#tymethod.sample_unchecked)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

Sample a point on this curve at the parameter value `t`, extracting the associated value. This is the unchecked version of sampling, which should only be used if the sample time `t` is already known to lie within the curve’s domain. [Read more](../../prelude/trait.Curve.html#tymethod.sample_unchecked)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#340)

#### fn [sample](../../prelude/trait.Curve.html#method.sample)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Sample a point on this curve at the parameter value `t`, returning `None` if the point is outside of the curve’s domain.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#349)

#### fn [sample\_clamped](../../prelude/trait.Curve.html#method.sample_clamped)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

Sample a point on this curve at the parameter value `t`, clamping `t` to lie inside the domain of the curve.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#764)

### impl<C, T> [CurveExt](../../prelude/trait.CurveExt.html "trait bevy::prelude::CurveExt")<T> for C

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#387)

#### fn [sample\_iter](../../prelude/trait.CurveExt.html#method.sample_iter)( &self, iter: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>

Sample a collection of `n >= 0` points on this curve at the parameter values `t_n`, returning `None` if the point is outside of the curve’s domain. [Read more](../../prelude/trait.CurveExt.html#method.sample_iter)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#402-405)

#### fn [sample\_iter\_unchecked](../../prelude/trait.CurveExt.html#method.sample_iter_unchecked)( &self, iter: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>

Sample a collection of `n >= 0` points on this curve at the parameter values `t_n`, extracting the associated values. This is the unchecked version of sampling, which should only be used if the sample times `t_n` are already known to lie within the curve’s domain. [Read more](../../prelude/trait.CurveExt.html#method.sample_iter_unchecked)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#415)

#### fn [sample\_iter\_clamped](../../prelude/trait.CurveExt.html#method.sample_iter_clamped)( &self, iter: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>

Sample a collection of `n >= 0` points on this curve at the parameter values `t_n`, clamping `t_n` to lie inside the domain of the curve. [Read more](../../prelude/trait.CurveExt.html#method.sample_iter_clamped)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#423-425)

#### fn [map](../../prelude/trait.CurveExt.html#method.map)<S, F>(self, f: F) -> [MapCurve](../../prelude/struct.MapCurve.html "struct bevy::prelude::MapCurve")<T, S, Self, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(T) -> S,

Create a new curve by mapping the values of this curve via a function `f`; i.e., if the sample at time `t` for this curve is `x`, the value at time `t` on the new curve will be `f(x)`.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#465-467)

#### fn [reparametrize](../../prelude/trait.CurveExt.html#method.reparametrize)<F>(self, domain: [Interval](../../prelude/struct.Interval.html "struct bevy::prelude::Interval"), f: F) -> [ReparamCurve](../../prelude/struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, Self, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html),

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") whose parameter space is related to the parameter space of this curve by `f`. For each time `t`, the sample from the new curve at time `t` is the sample from this curve at time `f(t)`. The given `domain` will be the domain of the new curve. The function `f` is expected to take `domain` into `self.domain()`. [Read more](../../prelude/trait.CurveExt.html#method.reparametrize)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#484-487)

#### fn [reparametrize\_linear](../../prelude/trait.CurveExt.html#method.reparametrize_linear)( self, domain: [Interval](../../prelude/struct.Interval.html "struct bevy::prelude::Interval"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[LinearReparamCurve](../../prelude/struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, Self>, [LinearReparamError](../../prelude/enum.LinearReparamError.html "enum bevy::prelude::LinearReparamError")\>

Linearly reparametrize this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve"), producing a new curve whose domain is the given `domain` instead of the current one. This operation is only valid for curves with bounded domains. [Read more](../../prelude/trait.CurveExt.html#method.reparametrize_linear)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#509-511)

#### fn [reparametrize\_by\_curve](../../prelude/trait.CurveExt.html#method.reparametrize_by_curve)<C>(self, other: C) -> [CurveReparamCurve](../../prelude/struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, Self, C>

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Reparametrize this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by sampling from another curve. [Read more](../../prelude/trait.CurveExt.html#method.reparametrize_by_curve)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#527)

#### fn [graph](../../prelude/trait.CurveExt.html#method.graph)(self) -> [GraphCurve](../../prelude/struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, Self>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") which is the graph of this one; that is, its output echoes the sample time as part of a tuple. [Read more](../../prelude/trait.CurveExt.html#method.graph)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#543-545)

#### fn [zip](../../prelude/trait.CurveExt.html#method.zip)<S, C>( self, other: C, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ZipCurve](../../prelude/struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<T, S, Self, C>, [InvalidIntervalError](../../prelude/interval/struct.InvalidIntervalError.html "struct bevy::prelude::interval::InvalidIntervalError")\>

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<S>,

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by zipping this curve together with another. [Read more](../../prelude/trait.CurveExt.html#method.zip)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#564-566)

#### fn [chain](../../prelude/trait.CurveExt.html#method.chain)<C>(self, other: C) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ChainCurve](../../prelude/struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, Self, C>, [ChainError](../../prelude/enum.ChainError.html "enum bevy::prelude::ChainError")\>

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T>,

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by composing this curve end-to-start with another, producing another curve with outputs of the same type. The domain of the other curve is translated so that its start coincides with where this curve ends. [Read more](../../prelude/trait.CurveExt.html#method.chain)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#589)

#### fn [reverse](../../prelude/trait.CurveExt.html#method.reverse)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ReverseCurve](../../prelude/struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, Self>, [ReverseError](../../prelude/enum.ReverseError.html "enum bevy::prelude::ReverseError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") inverting this curve on the x-axis, producing another curve with outputs of the same type, effectively playing backwards starting at `self.domain().end()` and transitioning over to `self.domain().start()`. The domain of the new curve is still the same. [Read more](../../prelude/trait.CurveExt.html#method.reverse)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#613)

#### fn [repeat](../../prelude/trait.CurveExt.html#method.repeat)(self, count: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[RepeatCurve](../../prelude/struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, Self>, [RepeatError](../../prelude/enum.RepeatError.html "enum bevy::prelude::RepeatError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") repeating this curve `N` times, producing another curve with outputs of the same type. The domain of the new curve will be bigger by a factor of `n + 1`. [Read more](../../prelude/trait.CurveExt.html#method.repeat)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#646)

#### fn [forever](../../prelude/trait.CurveExt.html#method.forever)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ForeverCurve](../../prelude/struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, Self>, [RepeatError](../../prelude/enum.RepeatError.html "enum bevy::prelude::RepeatError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") repeating this curve forever, producing another curve with outputs of the same type. The domain of the new curve will be unbounded. [Read more](../../prelude/trait.CurveExt.html#method.forever)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#663)

#### fn [ping\_pong](../../prelude/trait.CurveExt.html#method.ping_pong)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[PingPongCurve](../../prelude/struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, Self>, [PingPongError](../../prelude/enum.PingPongError.html "enum bevy::prelude::PingPongError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") chaining the original curve with its inverse, producing another curve with outputs of the same type. The domain of the new curve will be twice as long. The transition point is guaranteed to not make any jumps. [Read more](../../prelude/trait.CurveExt.html#method.ping_pong)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#688-691)

#### fn [chain\_continue](../../prelude/trait.CurveExt.html#method.chain_continue)<C>( self, other: C, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ContinuationCurve](../../prelude/struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, Self, C>, [ChainError](../../prelude/enum.ChainError.html "enum bevy::prelude::ChainError")\>

where T: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace"), C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T>,

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by composing this curve end-to-start with another, producing another curve with outputs of the same type. The domain of the other curve is translated so that its start coincides with where this curve ends. [Read more](../../prelude/trait.CurveExt.html#method.chain_continue)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#717)

#### fn [samples](../../prelude/trait.CurveExt.html#method.samples)( &self, samples: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

Extract an iterator over evenly-spaced samples from this curve. [Read more](../../prelude/trait.CurveExt.html#method.samples)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#750)

#### fn [by\_ref](../../prelude/trait.CurveExt.html#method.by_ref)(&self) -> &Self

Borrow this curve rather than taking ownership of it. This is essentially an alias for a prefix `&`; the point is that intermediate operations can be performed while retaining access to the original curve. [Read more](../../prelude/trait.CurveExt.html#method.by_ref)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#756-758)

#### fn [flip](../../prelude/trait.CurveExt.html#method.flip)<U, V>(self) -> impl [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[(V, U)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where Self: [CurveExt](../../prelude/trait.CurveExt.html "trait bevy::prelude::CurveExt")<[(U, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>,

Flip this curve so that its tuple output is arranged the other way.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#930)

### impl<C, T> [CurveResampleExt](../../prelude/trait.CurveResampleExt.html "trait bevy::prelude::CurveResampleExt")<T> for C

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#801-807)

#### fn [resample](../../prelude/trait.CurveResampleExt.html#method.resample)<I>( &self, segments: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), interpolation: I, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[SampleCurve](../../prelude/struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where I: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T,

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by interpolation over equally spaced sample values, using the provided `interpolation` to interpolate between adjacent samples. The curve is interpolated on `segments` segments between samples. For example, if `segments` is 1, only the start and end points of the curve are used as samples; if `segments` is 2, a sample at the midpoint is taken as well, and so on. [Read more](../../prelude/trait.CurveResampleExt.html#method.resample)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#830-832)

#### fn [resample\_auto](../../prelude/trait.CurveResampleExt.html#method.resample_auto)( &self, segments: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[SampleAutoCurve](../../prelude/struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where T: [StableInterpolate](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by interpolation over equally spaced sample values, using [automatic interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") to interpolate between adjacent samples. The curve is interpolated on `segments` segments between samples. For example, if `segments` is 1, only the start and end points of the curve are used as samples; if `segments` is 2, a sample at the midpoint is taken as well, and so on. [Read more](../../prelude/trait.CurveResampleExt.html#method.resample_auto)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#863-869)

#### fn [resample\_uneven](../../prelude/trait.CurveResampleExt.html#method.resample_uneven)<I>( &self, sample\_times: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, interpolation: I, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UnevenSampleCurve](../../prelude/struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where I: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T,

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by interpolation over samples taken at a given set of times. The given `interpolation` is used to interpolate adjacent samples, and the `sample_times` are expected to contain at least two valid times within the curve’s domain interval. [Read more](../../prelude/trait.CurveResampleExt.html#method.resample_uneven)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#905-910)

#### fn [resample\_uneven\_auto](../../prelude/trait.CurveResampleExt.html#method.resample_uneven_auto)( &self, sample\_times: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UnevenSampleAutoCurve](../../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where T: [StableInterpolate](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by [automatic interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") over samples taken at the given set of times. The given `sample_times` are expected to contain at least two valid times within the curve’s domain interval. [Read more](../../prelude/trait.CurveResampleExt.html#method.resample_uneven_auto)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#212-215)

### impl<T, C> [CurveWithDerivative](../../prelude/derivatives/trait.CurveWithDerivative.html "trait bevy::prelude::derivatives::CurveWithDerivative")<T> for C

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](../../prelude/derivatives/trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#217)

#### fn [with\_derivative](../../prelude/derivatives/trait.CurveWithDerivative.html#tymethod.with_derivative)(self) -> [SampleDerivativeWrapper](../../prelude/derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>

This curve, but with its first derivative included in sampling. [Read more](../../prelude/derivatives/trait.CurveWithDerivative.html#tymethod.with_derivative)

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#69)

### impl<In> [FromInput](../system/trait.FromInput.html "trait bevy::ecs::system::FromInput")<In> for In

where In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#71)

#### fn [from\_inner](../system/trait.FromInput.html#tymethod.from_inner)<'i>( inner: <In as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'i>, ) -> <In as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'i>

Converts the system input’s inner representation into this type’s inner representation.

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

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#65-67)

### impl<R> [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng") for R

where R: [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")<Error = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#70)

#### fn [next\_u32](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html#tymethod.next_u32)(&mut self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Return the next random `u32`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#77)

#### fn [next\_u64](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html#tymethod.next_u64)(&mut self) -> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

Return the next random `u64`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#84)

#### fn [fill\_bytes](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html#tymethod.fill_bytes)(&mut self, dst: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

Fill `dest` with random data. [Read more](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html#tymethod.fill_bytes)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#357)

### impl<R> [Rng](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html "trait rand::rng::Rng") for R

where R: [RngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#95-97)

#### fn [random](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random)<T>(&mut self) -> T

where [StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"): [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Return a random value via the [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform") distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#120-123)

#### fn [random\_iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_iter)<T>(self) -> [Iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html "struct rand::distr::distribution::Iter")<[StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"), Self, T> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"): [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Return an iterator over [`random`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random "method rand::rng::Rng::random") variates [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_iter)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#161-164)

#### fn [random\_range](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_range)<T, R>(&mut self, range: R) -> T

where T: [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform"), R: [SampleRange](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleRange.html "trait rand::distr::uniform::SampleRange")<T>,

Generate a random value in the given range. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_range)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#191)

#### fn [random\_bool](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_bool)(&mut self, p: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Return a bool with a probability `p` of being true. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_bool)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#225)

#### fn [random\_ratio](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_ratio)(&mut self, numerator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), denominator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Return a bool with a probability of `numerator/denominator` of being true. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_ratio)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#249)

#### fn [sample](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.sample)<T, D>(&mut self, distr: D) -> T

where D: [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Sample a new value, using the given distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.sample)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#286-289)

#### fn [sample\_iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.sample_iter)<T, D>(self, distr: D) -> [Iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html "struct rand::distr::distribution::Iter")<D, Self, T> [ⓘ](#)

where D: [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Create an iterator that generates values using the given distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.sample_iter)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#314)

#### fn [fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.fill)<T>(&mut self, dest: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where T: [Fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Fill.html "trait rand::rng::Fill") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fill any type implementing [`Fill`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Fill.html "trait rand::rng::Fill") with random data [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.fill)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#324-326)

#### fn [gen](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.gen)<T>(&mut self) -> T

where [StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"): [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

👎Deprecated since 0.9.0:

Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random "method rand::rng::Rng::random").

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#334-337)

#### fn [gen\_range](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.gen_range)<T, R>(&mut self, range: R) -> T

where T: [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform"), R: [SampleRange](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleRange.html "trait rand::distr::uniform::SampleRange")<T>,

👎Deprecated since 0.9.0:

Renamed to `random_range`

Alias for [`Rng::random_range`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_range "method rand::rng::Rng::random_range").

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#345)

#### fn [gen\_bool](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.gen_bool)(&mut self, p: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

👎Deprecated since 0.9.0:

Renamed to `random_bool`

Alias for [`Rng::random_bool`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_bool "method rand::rng::Rng::random_bool").

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#352)

#### fn [gen\_ratio](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.gen_ratio)(&mut self, numerator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), denominator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

👎Deprecated since 0.9.0:

Renamed to `random_ratio`

Alias for [`Rng::random_ratio`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_ratio "method rand::rng::Rng::random_ratio").

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#259)

### impl<R> [RngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore") for R

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#158-160)

### impl<T> [RngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore") for T

where T: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut"), <T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [RngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#163)

#### fn [next\_u32](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.next_u32)(&mut self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Return the next random `u32`. [Read more](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.next_u32)

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#168)

#### fn [next\_u64](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.next_u64)(&mut self) -> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

Return the next random `u64`. [Read more](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.next_u64)

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#173)

#### fn [fill\_bytes](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.fill_bytes)(&mut self, dst: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

Fill `dest` with random data. [Read more](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.fill_bytes)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#317)

### impl<R> [RngExt](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") for R

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#93-95)

#### fn [random](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random)<T>(&mut self) -> T

where [StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"): [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Return a random value via the [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform") distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#118-121)

#### fn [random\_iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_iter)<T>(self) -> [Iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html "struct rand::distr::distribution::Iter")<[StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"), Self, T> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"): [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Return an iterator over [`random`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random "method rand::rng::RngExt::random") variates [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_iter)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#159-162)

#### fn [random\_range](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_range)<T, R>(&mut self, range: R) -> T

where T: [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform"), R: [SampleRange](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleRange.html "trait rand::distr::uniform::SampleRange")<T>,

Generate a random value in the given range. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_range)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#189)

#### fn [random\_bool](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_bool)(&mut self, p: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Return a bool with a probability `p` of being true. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_bool)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#223)

#### fn [random\_ratio](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_ratio)(&mut self, numerator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), denominator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Return a bool with a probability of `numerator/denominator` of being true. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_ratio)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#247)

#### fn [sample](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.sample)<T, D>(&mut self, distr: D) -> T

where D: [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Sample a new value, using the given distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.sample)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#284-287)

#### fn [sample\_iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.sample_iter)<T, D>(self, distr: D) -> [Iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html "struct rand::distr::distribution::Iter")<D, Self, T> [ⓘ](#)

where D: [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Create an iterator that generates values using the given distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.sample_iter)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#312)

#### fn [fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.fill)<T>(&mut self, dest: &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where T: [Fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Fill.html "trait rand::rng::Fill"),

Fill any type implementing [`Fill`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Fill.html "trait rand::rng::Fill") with random data [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.fill)

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#100-104)

### impl<T, C, D> [SampleDerivative](../../prelude/derivatives/trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for D

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](../../prelude/derivatives/trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), D: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = C>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#106)

#### fn [sample\_with\_derivative\_unchecked](../../prelude/derivatives/trait.SampleDerivative.html#tymethod.sample_with_derivative_unchecked)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

Sample this curve at the parameter value `t`, extracting the associated value in addition to its derivative. This is the unchecked version of sampling, which should only be used if the sample time `t` is already known to lie within the curve’s domain. [Read more](../../prelude/derivatives/trait.SampleDerivative.html#tymethod.sample_with_derivative_unchecked)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#85)

#### fn [sample\_with\_derivative](../../prelude/derivatives/trait.SampleDerivative.html#method.sample_with_derivative)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>>

Sample this curve’s value and derivative at the parameter value `t`, returning `None` if the point is outside of the curve’s domain.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#94)

#### fn [sample\_with\_derivative\_clamped](../../prelude/derivatives/trait.SampleDerivative.html#method.sample_with_derivative_clamped)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

Sample this curve’s value and derivative at the parameter value `t`, clamping `t` to lie inside the domain of the curve.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#203-206)

### impl<T> [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source") for T

where T: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), <T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source"),

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#208)

#### type [Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice)<'a> = <<T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target") as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'a> where T: 'a

A type this `Source` can be sliced into.

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#213)

#### fn [len](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Length of the source

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#217-219)

#### fn [read](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.read)<'a, Chunk>(&'a self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Chunk>

where Chunk: [Chunk](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Chunk.html "trait logos::source::Chunk")<'a>,

Read a chunk of bytes into an array. Returns `None` when reading out of bounds would occur. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.read)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#224)

#### fn [slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice)(&self, range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'\_>>

Get a slice of the source at given range. This is analogous to `slice::get(range)`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#229)

#### unsafe fn [slice\_unchecked](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice_unchecked)( &self, range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> <T as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'\_>

Available on **non-crate feature `forbid_unsafe`** only.

Get a slice of the source at given range. This is analogous to `slice::get_unchecked(range)`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice_unchecked)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#233)

#### fn [is\_boundary](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.is_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Check if `index` is valid for this `Source`, that is: [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.is_boundary)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#237)

#### fn [find\_boundary](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#method.find_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

For `&str` sources attempts to find the closest `char` boundary at which source can be sliced, starting from `index`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#method.find_boundary)

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

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#251)

### impl<R> [TryCryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryCryptoRng.html "trait rand_core::TryCryptoRng") for R

where R: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut"), <R as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [TryCryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryCryptoRng.html "trait rand_core::TryCryptoRng"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#293)

### impl<R> [TryCryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryCryptoRng.html "trait rand_core::TryCryptoRng") for R

where R: [CryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.CryptoRng.html "trait rand_core::CryptoRng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

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

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#203-205)

### impl<R> [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng") for R

where R: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut"), <R as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#207)

#### type [Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error) = <<R as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target") as [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error "type rand_core::TryRng::Error")

The type returned in the event of a RNG error. [Read more](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error)

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#210)

#### fn [try\_next\_u32](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#tymethod.try_next_u32)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), <R as [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error "type rand_core::TryRng::Error")\>

Return the next random `u32`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#215)

#### fn [try\_next\_u64](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#tymethod.try_next_u64)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), <R as [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error "type rand_core::TryRng::Error")\>

Return the next random `u64`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#220)

#### fn [try\_fill\_bytes](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#tymethod.try_fill_bytes)(&mut self, dst: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), <R as [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error "type rand_core::TryRng::Error")\>

Fill `dst` entirely with random data.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#270)

### impl<R> [TryRngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html "trait rand_core::TryRngCore") for R

where R: [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#271)

#### type [Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#associatedtype.Error) = <R as [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error "type rand_core::TryRng::Error")

👎Deprecated since 0.10.0:

use `TryRng` instead

Error type.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#257)

### impl<R> [TryRngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html "trait rand_core::TryRngCore") for R

where R: [RngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#258)

#### type [Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a RNG error.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#261)

#### fn [try\_next\_u32](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#tymethod.try_next_u32)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), <R as [TryRngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html "trait rand_core::TryRngCore")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#associatedtype.Error "type rand_core::TryRngCore::Error")\>

Return the next random `u32`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#266)

#### fn [try\_next\_u64](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#tymethod.try_next_u64)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), <R as [TryRngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html "trait rand_core::TryRngCore")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#associatedtype.Error "type rand_core::TryRngCore::Error")\>

Return the next random `u64`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#271)

#### fn [try\_fill\_bytes](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes)( &mut self, dst: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), <R as [TryRngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html "trait rand_core::TryRngCore")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#associatedtype.Error "type rand_core::TryRngCore::Error")\>

Fill `dest` entirely with random data.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#232-234)

#### fn [unwrap\_err](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#method.unwrap_err)(self) -> [UnwrapErr](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.UnwrapErr.html "struct rand_core::UnwrapErr")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wrap RNG with the [`UnwrapErr`](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.UnwrapErr.html "struct rand_core::UnwrapErr") wrapper.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#240)

#### fn [unwrap\_mut](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#method.unwrap_mut)(&mut self) -> [UnwrapMut](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.UnwrapMut.html "struct rand_core::UnwrapMut")<'\_, Self>

Wrap RNG with the [`UnwrapMut`](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.UnwrapMut.html "struct rand_core::UnwrapMut") wrapper.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#246-248)

#### fn [read\_adapter](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#method.read_adapter)(&mut self) -> [RngReadAdapter](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.RngReadAdapter.html "struct rand_core::RngReadAdapter")<'\_, Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `std`** only.

Convert an [`RngCore`](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore") to a [`RngReadAdapter`](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.RngReadAdapter.html "struct rand_core::RngReadAdapter").

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Iter<D, Self, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html\\" title=\\"struct rand::distr::distribution::Iter\\">Iter</a>&lt;D, R, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;D, R, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html\\" title=\\"struct rand::distr::distribution::Iter\\">Iter</a>&lt;D, R, T&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html\\" title=\\"trait rand::distr::distribution::Distribution\\">Distribution</a>&lt;T&gt;,\\n R: <a class=\\"trait\\" href=\\"https://docs.rs/rand\_core/0.9.5/x86\_64-unknown-linux-gnu/rand\_core/trait.Rng.html\\" title=\\"trait rand\_core::Rng\\">Rng</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","Iter<StandardUniform, Self, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html\\" title=\\"struct rand::distr::distribution::Iter\\">Iter</a>&lt;D, R, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;D, R, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html\\" title=\\"struct rand::distr::distribution::Iter\\">Iter</a>&lt;D, R, T&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html\\" title=\\"trait rand::distr::distribution::Distribution\\">Distribution</a>&lt;T&gt;,\\n R: <a class=\\"trait\\" href=\\"https://docs.rs/rand\_core/0.9.5/x86\_64-unknown-linux-gnu/rand\_core/trait.Rng.html\\" title=\\"trait rand\_core::Rng\\">Rng</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}