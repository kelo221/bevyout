[bevy](../index.html)::[prelude](index.html)

# Function vw 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#563)

```rust
pub fn vw<T>(value: T) -> Valwhere
    T: ValNum,
```

Returns a [`Val::Vw`](enum.Val.html#variant.Vw "variant bevy::prelude::Val::Vw") representing a percentage of the viewport width.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/app/log\_layers\_ecs.rs ([line 132](../../src/log_layers_ecs/log_layers_ecs.rs.html#132))

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

Hide additional examples

examples/stress\_tests/many\_buttons.rs ([line 274](../../src/many_buttons/many_buttons.rs.html#274))

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

examples/testbed/ui.rs ([line 1726](../../src/testbed_ui/ui.rs.html#1726))

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
1848}
1849
1850mod boxed_content {
1851    use bevy::color::palettes::css::RED;
1852    use bevy::prelude::*;
1853
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
2033}
2034
2035mod editable_text {
2036    use bevy::color::palettes::css::YELLOW;
2037    use bevy::prelude::*;
2038    use bevy::text::EditableText;
2039    use bevy::text::TextEdit;
2040    use bevy::ui::widget::TextScroll;
2041
2042    pub fn setup(mut commands: Commands) {
2043        commands.spawn((Camera2d, DespawnOnExit(super::Scene::EditableText)));
2044        commands.spawn((
2045            Node {
2046                flex_direction: FlexDirection::Column,
2047                align_items: AlignItems::Center,
2048                justify_content: JustifyContent::Center,
2049                width: vw(100),
2050                height: vh(100),
2051                row_gap: px(25.),
2052                ..default()
2053            },
2054            DespawnOnExit(super::Scene::EditableText),
2055            children![
2056                (
2057                    EditableText {
2058                        pending_edits: vec![TextEdit::Insert("Single line EditableText".into())],
2059                        ..default()
2060                    },
2061                    Node {
2062                        width: px(200.),
2063                        border: px(2).all(),
2064                        ..default()
2065                    },
2066                    BorderColor::all(YELLOW),
2067                ),
2068                (
2069                    EditableText {
2070                        pending_edits: vec![
2071                            TextEdit::Insert(
2072                                "1. Multiline EditableText\n2.\n3.\n4.\n5.\n6.\n7.\n8.\n9.\n10."
2073                                    .into()
2074                            ),
2075                            TextEdit::TextStart(false),
2076                        ],
2077                        visible_lines: Some(8.),
2078                        ..default()
2079                    },
2080                    TextScroll::default(),
2081                    Node {
2082                        width: px(350.),
2083                        border: px(2).all(),
2084                        ..default()
2085                    },
2086                    BorderColor::all(YELLOW),
2087                ),
2088                (
2089                    EditableText {
2090                        pending_edits: vec![
2091                            TextEdit::Insert(
2092                                "1. Multiline EditableText\n2.\n3.\n4.\n5.\n6.\n7.\n8.\n9.\n10."
2093                                    .into()
2094                            ),
2095                            TextEdit::TextEnd(true),
2096                        ],
2097                        visible_lines: Some(8.),
2098                        ..default()
2099                    },
2100                    TextScroll::default(),
2101                    Node {
2102                        width: px(350.),
2103                        border: px(2).all(),
2104                        ..default()
2105                    },
2106                    BorderColor::all(YELLOW),
2107                ),
2108            ],
2109        ));
2110    }
```

examples/asset/asset\_saving.rs ([line 172](../../src/asset_saving/asset_saving.rs.html#172))

```rust
85fn setup(
86    mut commands: Commands,
87    asset_server: Res<AssetServer>,
88    mut images: ResMut<Assets<Image>>,
89) {
90    commands.spawn((
91        Camera2d,
92        Projection::Orthographic(OrthographicProjection {
93            scaling_mode: ScalingMode::FixedVertical {
94                viewport_height: 125.0,
95            },
96            ..OrthographicProjection::default_2d()
97        }),
98    ));
99
100    commands.spawn(Text(
101        r"Select a color from the palette at the bottom
102LMB - Draw with selected color
103F5 - Save image"
104            .into(),
105    ));
106
107    let handle = asset_server
108        .load_builder()
109        .with_settings(|settings: &mut ImageLoaderSettings| {
110            settings.sampler = ImageSampler::nearest();
111        })
112        .load(ASSET_PATH);
113    commands.spawn((
114        Sprite {
115            image: handle.clone(),
116            ..Default::default()
117        },
118        SpriteToSave,
119        Pickable::default(),
120    ));
121
122    // We're doing something a little cursed here: we initiate a load, and then insert a default
123    // image into that handle. If the load succeeds, the image will be replaced with the loaded
124    // contents. If it fails, the default image will remain. In real code, you likely want to poll
125    // `AssetServer::load_state` and only insert this on load failure.
126    images
127        .insert(&handle, {
128            let mut image = Image::new_fill(
129                Extent3d {
130                    width: 100,
131                    height: 100,
132                    depth_or_array_layers: 1,
133                },
134                TextureDimension::D2,
135                &[0, 0, 0, 255],
136                TextureFormat::Rgba8Unorm,
137                RenderAssetUsages::all(),
138            );
139            image.sampler = ImageSampler::nearest();
140            image
141        })
142        .unwrap();
143
144    commands.insert_resource(ImageToSave(handle));
145
146    let container = commands
147        .spawn((
148            Node {
149                width: percent(100),
150                height: percent(100),
151                align_items: AlignItems::End,
152                justify_content: JustifyContent::Center,
153                ..Default::default()
154            },
155            Pickable::IGNORE,
156        ))
157        .id();
158
159    for color in [
160        Color::WHITE,
161        Color::Srgba(tailwind::RED_500),
162        Color::Srgba(tailwind::ORANGE_500),
163        Color::Srgba(tailwind::YELLOW_500),
164        Color::Srgba(tailwind::GREEN_500),
165        Color::Srgba(tailwind::BLUE_500),
166        Color::Srgba(tailwind::INDIGO_500),
167        Color::Srgba(tailwind::VIOLET_500),
168        Color::BLACK,
169    ] {
170        let mut entity = commands.spawn((
171            Node {
172                width: vw(5),
173                height: vh(5),
174                border: px(5).all(),
175                ..Default::default()
176            },
177            SelectableColor,
178            BackgroundColor(color),
179            BorderColor::all(NORMAL_COLOR),
180            ChildOf(container),
181        ));
182        if color == Color::WHITE {
183            entity.insert((Selected, BorderColor::all(SELECTED_COLOR)));
184        }
185    }
186}
```

examples/ui/text/letter\_spacing.rs ([line 47](../../src/letter_spacing/letter_spacing.rs.html#47))

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