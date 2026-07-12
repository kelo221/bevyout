[bevy](../index.html)::[prelude](index.html)

# Enum Val 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#32)

```rust
pub enum Val {
    Auto,
    Px(f32),
    Percent(f32),
    Vw(f32),
    Vh(f32),
    VMin(f32),
    VMax(f32),
}
```

Represents the possible value types for layout properties.

This enum allows specifying values for various [`Node`](struct.Node.html "struct bevy::prelude::Node") properties in different units, such as logical pixels, percentages, or automatically determined values.

`Val` also implements [`core::str::FromStr`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") to allow parsing values from strings in the format `#.#px`. Whitespaces between the value and unit is allowed. The following units are supported:

*   `px`: logical pixels
*   `%`: percentage
*   `vw`: percentage of the viewport width
*   `vh`: percentage of the viewport height
*   `vmin`: percentage of the viewport’s smaller dimension
*   `vmax`: percentage of the viewport’s larger dimension

Additionally, `auto` will be parsed as [`Val::Auto`](enum.Val.html#variant.Auto "variant bevy::prelude::Val::Auto").

## Variants

### Auto

Automatically determine the value based on the context and other [`Node`](struct.Node.html "struct bevy::prelude::Node") properties.

### Px([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Set this value in logical pixels.

### Percent([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Set the value as a percentage of its parent node’s length along a specific axis.

If the UI node has no parent, the percentage is calculated based on the window’s length along the corresponding axis.

The chosen axis depends on the [`Node`](struct.Node.html "struct bevy::prelude::Node") field set:

*   For `flex_basis`, the percentage is relative to the main-axis length determined by the `flex_direction`.
*   For `gap`, `min_size`, `size`, and `max_size`:
    *   `width` is relative to the parent’s width.
    *   `height` is relative to the parent’s height.
*   For `margin`, `padding`, and `border` values: the percentage is relative to the parent node’s width.
*   For positions, `left` and `right` are relative to the parent’s width, while `bottom` and `top` are relative to the parent’s height.

### Vw([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Set this value in percent of the viewport width

### Vh([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Set this value in percent of the viewport height

### VMin([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Set this value in percent of the viewport’s smaller dimension.

### VMax([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Set this value in percent of the viewport’s larger dimension.

## Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#166)

### impl [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#167)

#### pub const [DEFAULT](#associatedconstant.DEFAULT): [Val](enum.Val.html "enum bevy::prelude::Val") = Self::Auto

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#168)

#### pub const [ZERO](#associatedconstant.ZERO): [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#186)

#### pub const fn [left](#method.left)(self) -> [UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

Returns a [`UiRect`](struct.UiRect.html "struct bevy::prelude::UiRect") with its `left` equal to this value, and all other fields set to `Val::ZERO`.

##### Example

```rust
let ui_rect = Val::Px(1.).left();

assert_eq!(ui_rect.left, Val::Px(1.));
assert_eq!(ui_rect.right, Val::ZERO);
assert_eq!(ui_rect.top, Val::ZERO);
assert_eq!(ui_rect.bottom, Val::ZERO);
```

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1949](../../src/testbed_ui/ui.rs.html#1949))

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

Hide additional examples

examples/ui/widgets/feathers\_gallery.rs ([line 425](../../src/feathers_gallery/feathers_gallery.rs.html#425))

```rust
103fn demo_column_1() -> impl Scene {
104    bsn! {
105        Node {
106            display: Display::Flex,
107            flex_direction: FlexDirection::Column,
108            align_items: AlignItems::Stretch,
109            justify_content: JustifyContent::Start,
110            padding: px(8),
111            row_gap: px(8),
112            width: percent(30),
113            min_width: px(200),
114        }
115        Children [
116            (
117                Node {
118                    display: Display::Flex,
119                    flex_direction: FlexDirection::Row,
120                    align_items: AlignItems::Center,
121                    justify_content: JustifyContent::Start,
122                    column_gap: px(8),
123                }
124                Children [
125                    (
126                        @FeathersButton {
127                            @caption: bsn! { Text("Normal") ThemedText }
128                        }
129                        Node {
130                            flex_grow: 1.0,
131                        }
132                        AccessibleLabel("Normal")
133                        on(|_activate: On<Activate>| {
134                            info!("Normal button clicked!");
135                        })
136                        AutoFocus
137                    ),
138                    (
139                        @FeathersButton {
140                            @caption: bsn! { Text("Disabled") ThemedText },
141                        }
142                        Node {
143                            flex_grow: 1.0,
144                        }
145                        AccessibleLabel("Disabled")
146                        InteractionDisabled
147                        DemoDisabledButton
148                        on(|_activate: On<Activate>| {
149                            info!("Disabled button clicked!");
150                        })
151                    ),
152                    (
153                        @FeathersButton {
154                            @caption: bsn! { Text("Primary") ThemedText },
155                            @variant: ButtonVariant::Primary,
156                        }
157                        AccessibleLabel("Primary")
158                        Node {
159                            flex_grow: 1.0,
160                        }
161                        on(|_activate: On<Activate>| {
162                            info!("Primary button clicked!");
163                        })
164                    ),
165                    (
166                        @FeathersMenu
167                        Children [
168                            (
169                                @FeathersMenuButton {
170                                    @caption: bsn! { Text("Menu") ThemedText }
171                                }
172                                AccessibleLabel("Menu Example")
173                                Node {
174                                    flex_grow: 1.0,
175                                }
176                            ),
177                            (
178                                @FeathersMenuPopup
179                                Children [
180                                    (
181                                        @FeathersMenuItem {
182                                            @caption: bsn! { Text("MenuItem 1") ThemedText }
183                                        }
184                                        on(|_: On<Activate>| {
185                                            info!("Menu item 1 clicked!");
186                                        })
187                                    ),
188                                    (
189                                        @FeathersMenuItem {
190                                            @caption: bsn! { Text("MenuItem 2") ThemedText }
191                                        }
192                                        on(|_: On<Activate>| {
193                                            info!("Menu item 2 clicked!");
194                                        })
195                                    ),
196                                    @FeathersMenuDivider,
197                                    (
198                                        @FeathersMenuItem {
199                                            @caption: bsn! { Text("MenuItem 3") ThemedText }
200                                        }
201                                        on(|_: On<Activate>| {
202                                            info!("Menu item 3 clicked!");
203                                        })
204                                    )
205                                ]
206                            )
207                        ]
208                    )
209                ]
210            ),
211            (
212                Node {
213                    display: Display::Flex,
214                    flex_direction: FlexDirection::Row,
215                    align_items: AlignItems::Center,
216                    justify_content: JustifyContent::Start,
217                    column_gap: px(1),
218                }
219                Children [
220                    (
221                        @FeathersButton {
222                            @caption: bsn! { Text("Left") ThemedText },
223                            @corners: RoundedCorners::Left,
224                        }
225                        Node {
226                            flex_grow: 1.0,
227                        }
228                        AccessibleLabel("Left")
229                        on(|_activate: On<Activate>| {
230                            info!("Left button clicked!");
231                        })
232                    ),
233                    (
234                        @FeathersButton {
235                            @caption: bsn! { Text("Center") ThemedText },
236                            @corners: RoundedCorners::None,
237                        }
238                        Node {
239                            flex_grow: 1.0,
240                        }
241                        AccessibleLabel("Center")
242                        on(|_activate: On<Activate>| {
243                            info!("Center button clicked!");
244                        })
245                    ),
246                    (
247                        @FeathersButton {
248                            @caption: bsn! { Text("Right") ThemedText },
249                            @variant: ButtonVariant::Primary,
250                            @corners: RoundedCorners::Right,
251                        }
252                        Node {
253                            flex_grow: 1.0,
254                        }
255                        AccessibleLabel("Right")
256                        on(|_activate: On<Activate>| {
257                            info!("Right button clicked!");
258                        })
259                    ),
260                ]
261            ),
262            (
263                @FeathersButton
264                on(|_activate: On<Activate>, mut ovr: ResMut<OverrideCursor>| {
265                    ovr.0 = if ovr.0.is_some() {
266                        None
267                    } else {
268                        Some(EntityCursor::System(SystemCursorIcon::Wait))
269                    };
270                    info!("Override cursor button clicked!");
271                })
272                Children [ (Text("Toggle override") ThemedText) ]
273            ),
274            (
275                @FeathersCheckbox {
276                    @caption: bsn! { Text("Checkbox") ThemedText }
277                }
278                Checked
279                AccessibleLabel("Checkbox Example")
280                on(
281                    |change: On<ValueChange<bool>>,
282                        query: Query<Entity, With<DemoDisabledButton>>,
283                        mut commands: Commands| {
284                        info!("Checkbox clicked!");
285                        let mut button = commands.entity(query.single().unwrap());
286                        if change.value {
287                            button.insert(InteractionDisabled);
288                        } else {
289                            button.remove::<InteractionDisabled>();
290                        }
291                        let mut checkbox = commands.entity(change.source);
292                        if change.value {
293                            checkbox.insert(Checked);
294                        } else {
295                            checkbox.remove::<Checked>();
296                        }
297                    }
298                )
299            ),
300            (
301                @FeathersCheckbox {
302                    @caption: bsn! { Text("Fast Click Checkbox") ThemedText }
303                }
304                ActivateOnPress
305                AccessibleLabel("Fast Click Checkbox Example")
306                on(
307                    |change: On<ValueChange<bool>>,
308                     mut commands: Commands| {
309                        info!("Checkbox clicked!");
310                        let mut checkbox = commands.entity(change.source);
311                        if change.value {
312                            checkbox.insert(Checked);
313                        } else {
314                            checkbox.remove::<Checked>();
315                        }
316                    }
317                )
318            ),
319            (
320                @FeathersCheckbox {
321                    @caption: bsn! { Text("Disabled") ThemedText },
322                }
323                InteractionDisabled
324                AccessibleLabel("Disabled Checkbox Example")
325                on(|_change: On<ValueChange<bool>>| {
326                    warn!("Disabled checkbox clicked!");
327                })
328            ),
329            (
330                @FeathersCheckbox {
331                    @caption: bsn! { Text("Checked+Disabled") ThemedText }
332                }
333                InteractionDisabled
334                Checked
335                AccessibleLabel("Disabled and Checked Checkbox Example")
336                on(|_change: On<ValueChange<bool>>| {
337                    warn!("Disabled checkbox clicked!");
338                })
339            ),
340            (
341                Node {
342                    display: Display::Flex,
343                    flex_direction: FlexDirection::Row,
344                    align_items: AlignItems::Center,
345                    justify_content: JustifyContent::Start,
346                    column_gap: px(8),
347                }
348                Children [
349                    (
350                        Node {
351                            display: Display::Flex,
352                            flex_direction: FlexDirection::Column,
353                            row_gap: px(4),
354                        }
355                        RadioGroup
356                        on(radio_self_update)
357                        Children [
358                            (
359                                @FeathersRadio {
360                                    @caption: bsn! { Text("One") ThemedText }
361                                }
362                                Checked
363                            ),
364                            @FeathersRadio {
365                                @caption: bsn! { Text("Two") ThemedText }
366                            },
367                            (
368                                @FeathersRadio {
369                                    @caption: bsn! { Text("Fast Click") ThemedText }
370                                }
371                                ActivateOnPress
372                            ),
373                            (
374                                @FeathersRadio {
375                                    @caption: bsn! { Text("Disabled") ThemedText }
376                                }
377                                InteractionDisabled
378                            ),
379                        ]
380                    )
381                ]
382            ),
383            (
384                Node {
385                    display: Display::Flex,
386                    flex_direction: FlexDirection::Row,
387                    align_items: AlignItems::Center,
388                    justify_content: JustifyContent::Start,
389                    column_gap: px(8),
390                }
391                Children [
392                    (@FeathersToggleSwitch on(checkbox_self_update)),
393                    (@FeathersToggleSwitch ActivateOnPress on(checkbox_self_update)),
394                    (@FeathersToggleSwitch InteractionDisabled on(checkbox_self_update)),
395                    (@FeathersToggleSwitch InteractionDisabled Checked on(checkbox_self_update)),
396                    (@FeathersDisclosureToggle on(checkbox_self_update)),
397                ]
398            ),
399            (
400                @FeathersSlider {
401                    @max: 100.0,
402                    @value: 20.0,
403                }
404                SliderStep(10.)
405                SliderPrecision(2)
406                on(slider_self_update)
407            ),
408            (
409                Node {
410                    display: Display::Flex,
411                    flex_direction: FlexDirection::Row,
412                    align_items: AlignItems::Center,
413                    justify_content: JustifyContent::SpaceBetween,
414                    column_gap: px(4),
415                }
416                Children [
417                    label("Srgba"),
418                    // Spacer
419                    flex_spacer(),
420                    // Text input
421                    (
422                        @FeathersTextInputContainer
423                        Node {
424                            flex_grow: 0.
425                            padding: { px(4).left() },
426                        }
427                        Children [
428                            (
429                                @FeathersTextInput {
430                                    @visible_width: 10f32,
431                                    @max_characters: 9usize,
432                                }
433                                InheritableFont {
434                                    font: fonts::MONO
435                                }
436                                HexColorInput
437                                on(handle_hex_color_change)
438                            )
439                        ]
440                    )
441                    (@FeathersColorSwatch SwatchType::Rgb),
442                ]
443            ),
444            (
445                @FeathersColorPlane::RedBlue
446                on(|change: On<ValueChange<Vec2>>, mut color: ResMut<DemoWidgetStates>| {
447                    color.rgb_color.red = change.value.x;
448                    color.rgb_color.blue = change.value.y;
449                })
450            ),
451            (
452                @FeathersColorSlider {
453                    @value: 0.5,
454                    @channel: ColorChannel::Red
455                }
456                AccessibleLabel("Red Channel")
457                on(|change: On<ValueChange<f32>>, mut color: ResMut<DemoWidgetStates>| {
458                    color.rgb_color.red = change.value;
459                })
460            ),
461            (
462                @FeathersColorSlider {
463                    @value: 0.5,
464                    @channel: ColorChannel::Green
465                }
466                AccessibleLabel("Green Channel")
467                on(|change: On<ValueChange<f32>>, mut color: ResMut<DemoWidgetStates>| {
468                    color.rgb_color.green = change.value;
469                })
470            ),
471            (
472                @FeathersColorSlider {
473                    @value: 0.5,
474                    @channel: ColorChannel::Blue
475                }
476                AccessibleLabel("Blue Channel")
477                on(|change: On<ValueChange<f32>>, mut color: ResMut<DemoWidgetStates>| {
478                    color.rgb_color.blue = change.value;
479                })
480            ),
481            (
482                @FeathersColorSlider {
483                    @value: 0.5,
484                    @channel: ColorChannel::Alpha
485                }
486                AccessibleLabel("Alpha Channel")
487                on(|change: On<ValueChange<f32>>, mut color: ResMut<DemoWidgetStates>| {
488                    color.rgb_color.alpha = change.value;
489                })
490            ),
491            (
492                Node {
493                    display: Display::Flex,
494                    align_items: AlignItems::Center,
495                    flex_direction: FlexDirection::Row,
496                    justify_content: JustifyContent::SpaceBetween,
497                }
498                Children [
499                    label("Hsl"),
500                    (@FeathersColorSwatch SwatchType::Hsl)
501                ]
502            ),
503            (
504                @FeathersColorSlider {
505                    @value: 0.5,
506                    @channel: ColorChannel::HslHue
507                }
508                AccessibleLabel("Hue Channel")
509                on(|change: On<ValueChange<f32>>, mut color: ResMut<DemoWidgetStates>| {
510                    color.hsl_color.hue = change.value;
511                })
512            ),
513            (
514                @FeathersColorSlider {
515                    @value: 0.5,
516                    @channel: ColorChannel::HslSaturation
517                }
518                AccessibleLabel("Saturation Channel")
519                on(|change: On<ValueChange<f32>>, mut color: ResMut<DemoWidgetStates>| {
520                    color.hsl_color.saturation = change.value;
521                })
522            ),
523            (
524                @FeathersColorSlider {
525                    @value: 0.5,
526                    @channel: ColorChannel::HslLightness
527                }
528                AccessibleLabel("Lightness Channel")
529                on(|change: On<ValueChange<f32>>, mut color: ResMut<DemoWidgetStates>| {
530                    color.hsl_color.lightness = change.value;
531                })
532            )
533        ]
534    }
535}
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#206)

#### pub const fn [right](#method.right)(self) -> [UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

Returns a [`UiRect`](struct.UiRect.html "struct bevy::prelude::UiRect") with its `right` equal to this value, and all other fields set to `Val::ZERO`.

##### Example

```rust
let ui_rect = Val::Px(1.).right();

assert_eq!(ui_rect.left, Val::ZERO);
assert_eq!(ui_rect.right, Val::Px(1.));
assert_eq!(ui_rect.top, Val::ZERO);
assert_eq!(ui_rect.bottom, Val::ZERO);
```

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/ui/layout/flex\_layout.rs ([line 50](../../src/flex_layout/flex_layout.rs.html#50))

```rust
21fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
22    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
23    commands.spawn(Camera2d);
24    commands
25        .spawn((
26            Node {
27                // fill the entire window
28                width: percent(100),
29                height: percent(100),
30                flex_direction: FlexDirection::Column,
31                align_items: AlignItems::Center,
32                padding: MARGIN.all(),
33                row_gap: MARGIN,
34                ..Default::default()
35            },
36            BackgroundColor(Color::BLACK),
37        ))
38        .with_children(|builder| {
39            // spawn the key
40            builder
41                .spawn(Node {
42                    flex_direction: FlexDirection::Row,
43                    ..default()
44                })
45                .with_children(|builder| {
46                    spawn_nested_text_bundle(
47                        builder,
48                        font.clone(),
49                        ALIGN_ITEMS_COLOR,
50                        MARGIN.right(),
51                        "AlignItems",
52                    );
53                    spawn_nested_text_bundle(
54                        builder,
55                        font.clone(),
56                        JUSTIFY_CONTENT_COLOR,
57                        UiRect::default(),
58                        "JustifyContent",
59                    );
60                });
61
62            builder
63                .spawn(Node {
64                    width: percent(100),
65                    height: percent(100),
66                    flex_direction: FlexDirection::Column,
67                    row_gap: MARGIN,
68                    ..default()
69                })
70                .with_children(|builder| {
71                    // spawn one child node for each combination of `AlignItems` and `JustifyContent`
72                    let justifications = [
73                        JustifyContent::FlexStart,
74                        JustifyContent::Center,
75                        JustifyContent::FlexEnd,
76                        JustifyContent::SpaceEvenly,
77                        JustifyContent::SpaceAround,
78                        JustifyContent::SpaceBetween,
79                    ];
80                    let alignments = [
81                        AlignItems::Baseline,
82                        AlignItems::FlexStart,
83                        AlignItems::Center,
84                        AlignItems::FlexEnd,
85                        AlignItems::Stretch,
86                    ];
87                    for align_items in alignments {
88                        builder
89                            .spawn(Node {
90                                width: percent(100),
91                                height: percent(100),
92                                flex_direction: FlexDirection::Row,
93                                column_gap: MARGIN,
94                                ..Default::default()
95                            })
96                            .with_children(|builder| {
97                                for justify_content in justifications {
98                                    spawn_child_node(
99                                        builder,
100                                        font.clone(),
101                                        align_items,
102                                        justify_content,
103                                    );
104                                }
105                            });
106                    }
107                });
108        });
109}
```

Hide additional examples

examples/testbed/ui.rs ([line 1965](../../src/testbed_ui/ui.rs.html#1965))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#226)

#### pub const fn [top](#method.top)(self) -> [UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

Returns a [`UiRect`](struct.UiRect.html "struct bevy::prelude::UiRect") with its `top` equal to this value, and all other fields set to `Val::ZERO`.

##### Example

```rust
let ui_rect = Val::Px(1.).top();

assert_eq!(ui_rect.left, Val::ZERO);
assert_eq!(ui_rect.right, Val::ZERO);
assert_eq!(ui_rect.top, Val::Px(1.));
assert_eq!(ui_rect.bottom, Val::ZERO);
```

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/ui/layout/flex\_layout.rs ([line 136](../../src/flex_layout/flex_layout.rs.html#136))

```rust
111fn spawn_child_node(
112    builder: &mut ChildSpawnerCommands,
113    font: Handle<Font>,
114    align_items: AlignItems,
115    justify_content: JustifyContent,
116) {
117    builder
118        .spawn((
119            Node {
120                flex_direction: FlexDirection::Column,
121                align_items,
122                justify_content,
123                width: percent(100),
124                height: percent(100),
125                ..default()
126            },
127            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
128        ))
129        .with_children(|builder| {
130            let labels = [
131                (format!("{align_items:?}"), ALIGN_ITEMS_COLOR, 0.),
132                (format!("{justify_content:?}"), JUSTIFY_CONTENT_COLOR, 3.),
133            ];
134            for (text, color, top_margin) in labels {
135                // We nest the text within a parent node because margins and padding can't be directly applied to text nodes currently.
136                spawn_nested_text_bundle(builder, font.clone(), color, px(top_margin).top(), &text);
137            }
138        });
139}
```

Hide additional examples

examples/ui/text/multiple\_text\_inputs.rs ([line 167](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#167))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#246)

#### pub const fn [bottom](#method.bottom)(self) -> [UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

Returns a [`UiRect`](struct.UiRect.html "struct bevy::prelude::UiRect") with its `bottom` equal to this value, and all other fields set to `Val::ZERO`.

##### Example

```rust
let ui_rect = Val::Px(1.).bottom();

assert_eq!(ui_rect.left, Val::ZERO);
assert_eq!(ui_rect.right, Val::ZERO);
assert_eq!(ui_rect.top, Val::ZERO);
assert_eq!(ui_rect.bottom, Val::Px(1.));
```

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/ui/text/multiple\_text\_inputs.rs ([line 74](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#74))

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

Hide additional examples

examples/ui/text/letter\_spacing.rs ([line 62](../../src/letter_spacing/letter_spacing.rs.html#62))

```rust
30fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
31    commands.spawn(Camera2d);
32
33    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
34
35    commands
36        .spawn(Node {
37            width: percent(100),
38            height: percent(100),
39            ..default()
40        })
41        .with_children(|parent| {
42            parent
43                .spawn(Node {
44                    width: percent(100),
45                    height: percent(100),
46                    align_items: AlignItems::Center,
47                    padding: UiRect::axes(vw(5), vh(10)),
48                    row_gap: vh(6),
49                    flex_direction: FlexDirection::Column,
50                    ..default()
51                })
52                .with_children(|parent| {
53                    parent.spawn((
54                        Text::new("HELLO"),
55                        Underline,
56                        TextFont {
57                            font: font.clone().into(),
58                            font_size: FontSize::Vh(6.0),
59                            ..default()
60                        },
61                        Node {
62                            padding: vh(2).bottom(),
63                            ..default()
64                        },
65                    ));
66
67                    // Left justified
68                    parent
69                        .spawn(Node {
70                            flex_direction: FlexDirection::Column,
71                            width: percent(100.0),
72                            ..default()
73                        })
74                        .with_children(|parent| {
75                            parent.spawn((
76                                Text::new("Justify::Left"),
77                                TextFont {
78                                    font: font.clone().into(),
79                                    font_size: FontSize::Vh(2.0),
80                                    ..default()
81                                },
82                            ));
83                            parent.spawn((
84                                Text::new("letter spacing"),
85                                AnimatedLetterSpacing,
86                                TextLayout::justify(Justify::Left),
87                                TextFont {
88                                    font: font.clone().into(),
89                                    font_size: FontSize::Vh(6.0),
90                                    ..default()
91                                },
92                                Node {
93                                    width: percent(100.0),
94                                    ..default()
95                                },
96                                // Custom `LetterSpacing` can be added to any text entity as a component
97                                LetterSpacing::Px(0.0),
98                            ));
99                        });
100
101                    // Center justified
102                    parent
103                        .spawn(Node {
104                            flex_direction: FlexDirection::Column,
105                            width: percent(100.0),
106                            ..default()
107                        })
108                        .with_children(|parent| {
109                            parent.spawn((
110                                Text::new("Justify::Center"),
111                                TextFont {
112                                    font: font.clone().into(),
113                                    font_size: FontSize::Vh(2.0),
114                                    ..default()
115                                },
116                            ));
117                            parent.spawn((
118                                Text::new("letter spacing"),
119                                AnimatedLetterSpacing,
120                                TextLayout::justify(Justify::Center),
121                                TextFont {
122                                    font: font.clone().into(),
123                                    font_size: FontSize::Vh(6.0),
124                                    ..default()
125                                },
126                                Node {
127                                    width: percent(100.0),
128                                    ..default()
129                                },
130                                // Custom `LetterSpacing` can be added to any text entity as a component
131                                LetterSpacing::Px(0.0),
132                            ));
133                        });
134
135                    // Right justified
136                    parent
137                        .spawn(Node {
138                            flex_direction: FlexDirection::Column,
139                            width: percent(100.0),
140                            ..default()
141                        })
142                        .with_children(|parent| {
143                            parent.spawn((
144                                Text::new("Justify::Right"),
145                                TextFont {
146                                    font: font.clone().into(),
147                                    font_size: FontSize::Vh(2.0),
148                                    ..default()
149                                },
150                            ));
151                            parent.spawn((
152                                Text::new("letter spacing"),
153                                AnimatedLetterSpacing,
154                                TextLayout::justify(Justify::Right),
155                                TextFont {
156                                    font: font.clone().into(),
157                                    font_size: FontSize::Vh(6.0),
158                                    ..default()
159                                },
160                                Node {
161                                    width: percent(100.0),
162                                    ..default()
163                                },
164                                // Custom `LetterSpacing` can be added to any text entity as a component
165                                LetterSpacing::Px(0.0),
166                            ));
167                        });
168                });
169
170            parent.spawn((
171                Text::new("LetterSpacing::Px(0.0)"),
172                LetterSpacingLabel,
173                TextFont {
174                    font: font.clone().into(),
175                    font_size: FontSize::Vh(3.0),
176                    ..default()
177                },
178                Node {
179                    position_type: PositionType::Absolute,
180                    bottom: vh(2.0),
181                    left: vw(2.0),
182                    ..default()
183                },
184            ));
185
186            parent.spawn((
187                Text::new("← → to adjust   Space to toggle Px / Rem"),
188                TextFont {
189                    font: font.clone().into(),
190                    font_size: FontSize::Vh(2.5),
191                    ..default()
192                },
193                Node {
194                    position_type: PositionType::Absolute,
195                    bottom: vh(2.0),
196                    right: vw(2.0),
197                    ..default()
198                },
199            ));
200        });
201}
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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#264)

#### pub const fn [all](#method.all)(self) -> [UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

Returns a [`UiRect`](struct.UiRect.html "struct bevy::prelude::UiRect") with all its fields equal to this value.

##### Example

```rust
let ui_rect = Val::Px(1.).all();

assert_eq!(ui_rect.left, Val::Px(1.));
assert_eq!(ui_rect.right, Val::Px(1.));
assert_eq!(ui_rect.top, Val::Px(1.));
assert_eq!(ui_rect.bottom, Val::Px(1.));
```

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/ui/text/editable\_text\_filter.rs ([line 31](../../src/editable_text_filter/editable_text_filter.rs.html#31))

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

Hide additional examples

examples/ui/images/image\_node.rs ([line 33](../../src/image_node/image_node.rs.html#33))

```rust
13fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
14    // Ui camera
15    commands.spawn(Camera2d);
16
17    commands.spawn((
18        // This root Node serves as a container for the ImageNode.
19        // In this case, it will center the item on the screen.
20        Node {
21            width: percent(100),
22            height: percent(100),
23            align_items: AlignItems::Center,
24            justify_content: JustifyContent::Center,
25            ..default()
26        },
27        // Child Nodes are added with the `children!` macro.
28        children![(
29            // Create a new `ImageNode` with the given texture.
30            ImageNode::new(asset_server.load("branding/icon.png")),
31            // Child Node control `ImageNode` size
32            Node {
33                border: px(5.).all(),
34                padding: px(10.).all(),
35                width: px(256.),
36                height: px(256.),
37                ..default()
38            },
39            BorderColor::all(Color::WHITE),
40        )],
41    ));
42}
```

examples/ui/text/font\_weights.rs ([line 38](../../src/font_weights/font_weights.rs.html#38))

```rust
12fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
13    let font: FontSource = asset_server.load("fonts/MonaSans-VariableFont.ttf").into();
14
15    commands.spawn(Camera2d);
16
17    commands.spawn((
18        Node {
19            flex_direction: FlexDirection::Column,
20            align_self: AlignSelf::Center,
21            justify_self: JustifySelf::Center,
22            align_items: AlignItems::Center,
23            ..default()
24        },
25        children![
26            (
27                Text::new("Font Weights"),
28                TextFont {
29                    font: font.clone(),
30                    font_size: FontSize::Px(32.0),
31                    ..default()
32                },
33                Underline,
34            ),
35            (
36                Node {
37                    flex_direction: FlexDirection::Column,
38                    padding: px(8.).all(),
39                    row_gap: px(8.),
40                    ..default()
41                },
42                Children::spawn(SpawnIter(
43                    [100, 134, 200, 300, 400, 500, 600, 700, 800, 900, 950]
44                        .into_iter()
45                        .map(move |weight| (
46                            Text(format!("Weight {weight}")),
47                            TextFont {
48                                font: font.clone(),
49                                font_size: FontSize::Px(32.0),
50                                weight: FontWeight(weight),
51                                ..default()
52                            },
53                        ))
54                ))
55            ),
56        ],
57    ));
58}
```

examples/ui/text/font\_variations.rs ([line 39](../../src/font_variations/font_variations.rs.html#39))

```rust
13fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
14    let font: FontSource = asset_server.load("fonts/MonaSans-VariableFont.ttf").into();
15
16    commands.spawn(Camera2d);
17
18    commands.spawn((
19        Node {
20            flex_direction: FlexDirection::Column,
21            align_self: AlignSelf::Center,
22            justify_self: JustifySelf::Center,
23            align_items: AlignItems::Center,
24            ..default()
25        },
26        children![
27            (
28                Text::new("Font Variations (wght axis)"),
29                TextFont {
30                    font: font.clone(),
31                    font_size: FontSize::Px(32.0),
32                    ..default()
33                },
34                Underline,
35            ),
36            (
37                Node {
38                    flex_direction: FlexDirection::Column,
39                    padding: px(8.).all(),
40                    row_gap: px(8.),
41                    ..default()
42                },
43                Children::spawn(SpawnIter(
44                    [100, 200, 300, 400, 500, 600, 700, 800, 900]
45                        .into_iter()
46                        .map(move |weight| (
47                            Text(format!("wght {weight}")),
48                            TextFont {
49                                font: font.clone(),
50                                font_size: FontSize::Px(32.0),
51                                font_variations: FontVariations::builder()
52                                    .set(FontVariationTag::WEIGHT, weight as f32)
53                                    .build(),
54                                ..default()
55                            },
56                        ))
57                )),
58            ),
59        ],
60    ));
61}
```

examples/ui/text/text\_input.rs ([line 54](../../src/text_input/text_input.rs.html#54))

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
117
118fn build_input_text(commands: &mut Commands, is_left: bool, font_size: f32) -> Entity {
119    commands
120        .spawn((
121            Node {
122                border: px(2).all(),
123                ..Default::default()
124            },
125            BorderColor::from(Color::from(SLATE_300)),
126            Name::new(if is_left { "Left" } else { "Right" }),
127            EditableText {
128                visible_width: Some(10.),
129                allow_newlines: false,
130                ..Default::default()
131            },
132            TextLayout::no_wrap(),
133            TextFont {
134                font_size: FontSize::Px(font_size),
135                ..default()
136            },
137            TextCursorStyle::default(),
138            TabIndex(if is_left { 0 } else { 1 }),
139            BackgroundColor(DARK_GREY.into()),
140        ))
141        .id()
142}
```

examples/ui/text/ime\_support.rs ([line 48](../../src/ime_support/ime_support.rs.html#48))

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

Additional examples can be found in:  

*   [examples/ui/text/text\_background\_colors.rs](../../src/text_background_colors/text_background_colors.rs.html#45)
*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#186)
*   [examples/ui/layout/anchor\_layout.rs](../../src/anchor_layout/anchor_layout.rs.html#103)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#97)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#174)
*   [examples/ui/text/system\_fonts.rs](../../src/system_fonts/system_fonts.rs.html#65)
*   [examples/ui/layout/flex\_layout.rs](../../src/flex_layout/flex_layout.rs.html#32)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../src/scrollbars/scrollbars.rs.html#116)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#184)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#95)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#97)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#40)
*   [examples/ui/styling/borders.rs](../../src/borders/borders.rs.html#94)
*   [examples/ui/text/font\_query.rs](../../src/font_query/font_query.rs.html#25)
*   [examples/ui/text/multiline\_text\_input.rs](../../src/multiline_text_input/multiline_text_input.rs.html#55)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#283)

#### pub const fn [horizontal](#method.horizontal)(self) -> [UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

Returns a [`UiRect`](struct.UiRect.html "struct bevy::prelude::UiRect") with all its `left` and `right` equal to this value, and its `top` and `bottom` set to `Val::ZERO`.

##### Example

```rust
let ui_rect = Val::Px(1.).horizontal();

assert_eq!(ui_rect.left, Val::Px(1.));
assert_eq!(ui_rect.right, Val::Px(1.));
assert_eq!(ui_rect.top, Val::ZERO);
assert_eq!(ui_rect.bottom, Val::ZERO);
```

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/ui/layout/anchor\_layout.rs ([line 35](../../src/anchor_layout/anchor_layout.rs.html#35))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#302)

#### pub const fn [vertical](#method.vertical)(self) -> [UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

Returns a [`UiRect`](struct.UiRect.html "struct bevy::prelude::UiRect") with all its `top` and `bottom` equal to this value, and its `left` and `right` set to `Val::ZERO`.

##### Example

```rust
let ui_rect = Val::Px(1.).vertical();

assert_eq!(ui_rect.left, Val::ZERO);
assert_eq!(ui_rect.right, Val::ZERO);
assert_eq!(ui_rect.top, Val::Px(1.));
assert_eq!(ui_rect.bottom, Val::Px(1.));
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#318)

#### pub const fn [try\_add](#method.try_add)(self, val: [Val](enum.Val.html "enum bevy::prelude::Val")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Val](enum.Val.html "enum bevy::prelude::Val"), [ValArithmeticError](enum.ValArithmeticError.html "enum bevy::prelude::ValArithmeticError")\>

Try to add two `Val`s.

Returns [`ValArithmeticError::IncompatibleUnits`](enum.ValArithmeticError.html#variant.IncompatibleUnits "variant bevy::prelude::ValArithmeticError::IncompatibleUnits") if the units differ or both are `Val::Auto`.

```rust
assert_eq!(Val::Px(1.).try_add(Val::Px(2.)), Ok(Val::Px(3.)));
assert_eq!(
    Val::Px(1.).try_add(Val::Percent(2.)),
    Err(ValArithmeticError::IncompatibleUnits)
);
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#342)

#### pub const fn [try\_sub](#method.try_sub)(self, val: [Val](enum.Val.html "enum bevy::prelude::Val")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Val](enum.Val.html "enum bevy::prelude::Val"), [ValArithmeticError](enum.ValArithmeticError.html "enum bevy::prelude::ValArithmeticError")\>

Try to subtract one `Val` from another.

Returns [`ValArithmeticError::IncompatibleUnits`](enum.ValArithmeticError.html#variant.IncompatibleUnits "variant bevy::prelude::ValArithmeticError::IncompatibleUnits") if the units differ or both are `Val::Auto`.

```rust
assert_eq!(Val::Px(3.).try_sub(Val::Px(2.)), Ok(Val::Px(1.)));
assert_eq!(
    Val::Px(1.).try_sub(Val::Percent(2.)),
    Err(ValArithmeticError::IncompatibleUnits)
);
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#445)

### impl [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#450-455)

#### pub const fn [resolve](#method.resolve)( self, scale\_factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), physical\_base\_value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), physical\_target\_size: [Vec2](struct.Vec2.html "struct bevy::prelude::Vec2"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [ValArithmeticError](enum.ValArithmeticError.html "enum bevy::prelude::ValArithmeticError")\>

Resolves this [`Val`](enum.Val.html "enum bevy::prelude::Val") to a value in physical pixels from the given `scale_factor`, `physical_base_value`, and `physical_target_size` context values.

Returns a [`ValArithmeticError::NonEvaluable`](enum.ValArithmeticError.html#variant.NonEvaluable "variant bevy::prelude::ValArithmeticError::NonEvaluable") if the [`Val`](enum.Val.html "enum bevy::prelude::Val") is impossible to resolve into a concrete value.

## Trait Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Val](enum.Val.html "enum bevy::prelude::Val")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#355)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#356)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Val](enum.Val.html "enum bevy::prelude::Val")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#29)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#29)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Val](enum.Val.html "enum bevy::prelude::Val"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#391)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#392)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Val](enum.Val.html "enum bevy::prelude::Val")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#394)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> <[Val](enum.Val.html "enum bevy::prelude::Val") as [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output "type core::ops::arith::Div::Output")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#407)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#408)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [Enum](../reflect/enums/trait.Enum.html "trait bevy::reflect::enums::Enum") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [field](../reflect/enums/trait.Enum.html#tymethod.field)(&self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field (in the current variant) with the given name. [Read more](../reflect/enums/trait.Enum.html#tymethod.field)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [field\_at](../reflect/enums/trait.Enum.html#tymethod.field_at)( &self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field (in the current variant) at the given index.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [field\_mut](../reflect/enums/trait.Enum.html#tymethod.field_mut)( &mut self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field (in the current variant) with the given name. [Read more](../reflect/enums/trait.Enum.html#tymethod.field_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [field\_at\_mut](../reflect/enums/trait.Enum.html#tymethod.field_at_mut)( &mut self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field (in the current variant) at the given index.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [index\_of](../reflect/enums/trait.Enum.html#tymethod.index_of)(&self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Returns the index of the field (in the current variant) with the given name. [Read more](../reflect/enums/trait.Enum.html#tymethod.index_of)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [name\_at](../reflect/enums/trait.Enum.html#tymethod.name_at)(&self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the field (in the current variant) with the given index. [Read more](../reflect/enums/trait.Enum.html#tymethod.name_at)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [iter\_fields](../reflect/enums/trait.Enum.html#tymethod.iter_fields)(&self) -> [VariantFieldIter](../reflect/enums/struct.VariantFieldIter.html "struct bevy::reflect::enums::VariantFieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the current variant’s fields.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [field\_len](../reflect/enums/trait.Enum.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the current variant.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [variant\_name](../reflect/enums/trait.Enum.html#tymethod.variant_name)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

The name of the current variant.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [variant\_index](../reflect/enums/trait.Enum.html#tymethod.variant_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

The index of the current variant.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [variant\_type](../reflect/enums/trait.Enum.html#tymethod.variant_type)(&self) -> [VariantType](../reflect/enums/enum.VariantType.html "enum bevy::reflect::enums::VariantType")

The type of the current variant.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [to\_dynamic\_enum](../reflect/enums/trait.Enum.html#method.to_dynamic_enum)(&self) -> [DynamicEnum](../reflect/enums/struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum")

Creates a new [`DynamicEnum`](../reflect/enums/struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum") from this enum.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#133)

#### fn [is\_variant](../reflect/enums/trait.Enum.html#method.is_variant)(&self, variant\_type: [VariantType](../reflect/enums/enum.VariantType.html "enum bevy::reflect::enums::VariantType")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the current variant’s type matches the given one.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#137)

#### fn [variant\_path](../reflect/enums/trait.Enum.html#method.variant_path)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Returns the full path to the current variant.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#144)

#### fn [get\_represented\_enum\_info](../reflect/enums/trait.Enum.html#method.get_represented_enum_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [EnumInfo](../reflect/enums/struct.EnumInfo.html "struct bevy::reflect::enums::EnumInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#472)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Val](enum.Val.html "enum bevy::prelude::Val")\> for [FontSize](enum.FontSize.html "enum bevy::prelude::FontSize")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#473)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Val](enum.Val.html "enum bevy::prelude::Val")) -> [FontSize](enum.FontSize.html "enum bevy::prelude::FontSize")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#987)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Val](enum.Val.html "enum bevy::prelude::Val")\> for [UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#988)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Val](enum.Val.html "enum bevy::prelude::Val")) -> [UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#1154)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Val](enum.Val.html "enum bevy::prelude::Val")\> for [UiPosition](struct.UiPosition.html "struct bevy::prelude::UiPosition")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#1155)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(x: [Val](enum.Val.html "enum bevy::prelude::Val")) -> [UiPosition](struct.UiPosition.html "struct bevy::prelude::UiPosition")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2793)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Val](enum.Val.html "enum bevy::prelude::Val")\> for [BorderRadius](struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2794)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Val](enum.Val.html "enum bevy::prelude::Val")) -> [BorderRadius](struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Val](enum.Val.html "enum bevy::prelude::Val")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Val](enum.Val.html "enum bevy::prelude::Val") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [from\_reflect](trait.FromReflect.html#tymethod.from_reflect)(\_\_param0: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Val](enum.Val.html "enum bevy::prelude::Val")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#79)

### impl [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#80)

#### type [Err](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err) = [ValParseError](enum.ValParseError.html "enum bevy::prelude::ValParseError")

The associated error which can be returned from parsing.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#82)

#### fn [from\_str](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)(s: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Val](enum.Val.html "enum bevy::prelude::Val"), <[Val](enum.Val.html "enum bevy::prelude::Val") as [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr")\>::[Err](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err "type core::str::traits::FromStr::Err")\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Val](enum.Val.html "enum bevy::prelude::Val"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#361)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#362)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Val](enum.Val.html "enum bevy::prelude::Val")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#364)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> <[Val](enum.Val.html "enum bevy::prelude::Val") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#377)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#378)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#421)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#422)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Val](enum.Val.html "enum bevy::prelude::Val")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#424)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> <[Val](enum.Val.html "enum bevy::prelude::Val") as [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output "type core::ops::arith::Neg::Output")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#124)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#125)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Val](enum.Val.html "enum bevy::prelude::Val")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [get\_represented\_type\_info](trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [try\_apply](trait.PartialReflect.html#tymethod.try_apply)( &mut self, \_\_value\_param: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [reflect\_kind](trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [reflect\_ref](trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [reflect\_mut](trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [reflect\_owned](trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Val](enum.Val.html "enum bevy::prelude::Val")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [try\_into\_reflect](trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Val](enum.Val.html "enum bevy::prelude::Val")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [try\_as\_reflect](trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [try\_as\_reflect\_mut](trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [into\_partial\_reflect](trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Val](enum.Val.html "enum bevy::prelude::Val")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [as\_partial\_reflect](trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [as\_partial\_reflect\_mut](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [reflect\_hash](trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#26)

#### fn [reflect\_partial\_eq](trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [reflect\_partial\_cmp](trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#26)

#### fn [debug](trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#26)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [into\_any](trait.Reflect.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Val](enum.Val.html "enum bevy::prelude::Val")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [as\_any](trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [as\_any\_mut](trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [into\_reflect](trait.Reflect.html#tymethod.into_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Val](enum.Val.html "enum bevy::prelude::Val")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [as\_reflect](trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [as\_reflect\_mut](trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [set](trait.Reflect.html#tymethod.set)(&mut self, value: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#29)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#29)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#486)

### impl [TryStableInterpolate](../math/trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#496)

#### fn [try\_interpolate\_stable](../math/trait.TryStableInterpolate.html#tymethod.try_interpolate_stable)( &self, other: &[Val](enum.Val.html "enum bevy::prelude::Val"), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Val](enum.Val.html "enum bevy::prelude::Val"), <[Val](enum.Val.html "enum bevy::prelude::Val") as [TryStableInterpolate](../math/trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate")\>::[Error](../math/trait.TryStableInterpolate.html#associatedtype.Error "type bevy::math::TryStableInterpolate::Error")\>

##### Example

```rust
assert!(matches!(Val::Px(0.0).try_interpolate_stable(&Val::Px(10.0), 0.5), Ok(Val::Px(5.0))));
```

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#487)

#### type [Error](../math/trait.TryStableInterpolate.html#associatedtype.Error) = [MismatchedUnitsError](../math/struct.MismatchedUnitsError.html "struct bevy::math::MismatchedUnitsError")

Error produced when the value cannot be interpolated.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [type\_path](trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [short\_type\_path](trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [type\_ident](trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [crate\_name](trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [module\_path](trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Val](enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Val](enum.Val.html "enum bevy::prelude::Val")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Val](enum.Val.html "enum bevy::prelude::Val")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Val](enum.Val.html "enum bevy::prelude::Val")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Val](enum.Val.html "enum bevy::prelude::Val")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Val](enum.Val.html "enum bevy::prelude::Val")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Val](enum.Val.html "enum bevy::prelude::Val")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Val](enum.Val.html "enum bevy::prelude::Val")

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","VariantFieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/enums/struct.VariantFieldIter.html\\" title=\\"struct bevy::reflect::enums::VariantFieldIter\\">VariantFieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/enums/struct.VariantFieldIter.html\\" title=\\"struct bevy::reflect::enums::VariantFieldIter\\">VariantFieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"enum\\" href=\\"../reflect/enums/enum.VariantField.html\\" title=\\"enum bevy::reflect::enums::VariantField\\">VariantField</a>&lt;'a&gt;;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}