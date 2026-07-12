[bevy](../index.html)::[prelude](index.html)

# Struct RepeatedGridTrack 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1830)

```rust
pub struct RepeatedGridTrack { /* private fields */ }
```

Represents a _possibly_ repeated [`GridTrack`](struct.GridTrack.html "struct bevy::prelude::GridTrack").

The repetition parameter can either be:

*   The integer `1`, in which case the track is non-repeated.
*   a `u16` count to repeat the track N times.
*   A `GridTrackRepetition::AutoFit` or `GridTrackRepetition::AutoFill`.

Note: that in the common case you want a non-repeating track (repetition count 1), you may use the constructor methods on [`GridTrack`](struct.GridTrack.html "struct bevy::prelude::GridTrack") to create a `RepeatedGridTrack`. i.e. `GridTrack::px(10.0)` is equivalent to `RepeatedGridTrack::px(1, 10.0)`.

You may only use one auto-repetition per track list. And if your track list contains an auto repetition then all tracks (in and outside of the repetition) must be fixed size (px or percent). Integer repetitions are just shorthand for writing out N tracks longhand and are not subject to the same limitations.

## Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1835)

### impl [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1837)

#### pub fn [px](#method.px)<T>(repetition: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")\>, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with a fixed pixel size

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/overflow\_debug.rs ([line 118](../../src/overflow_debug/overflow_debug.rs.html#118))

```rust
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
```

Hide additional examples

examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs ([line 69](../../src/drag_to_scroll/drag_to_scroll.rs.html#69))

```rust
22fn setup(mut commands: Commands) {
23    let w = 60;
24    let h = 40;
25
26    commands.spawn(Camera2d);
27    commands.insert_resource(UiScale(0.5));
28
29    commands
30        .spawn((
31            Node {
32                width: percent(100),
33                height: percent(100),
34                overflow: Overflow::scroll(),
35                ..Default::default()
36            },
37            ScrollPosition(Vec2::ZERO),
38            ScrollableNode,
39            ScrollStart(Vec2::ZERO),
40        ))
41        .observe(
42            |drag: On<Pointer<Drag>>,
43             ui_scale: Res<UiScale>,
44             mut scroll_position_query: Query<
45                (&mut ScrollPosition, &ScrollStart),
46                With<ScrollableNode>,
47            >| {
48                if let Ok((mut scroll_position, start)) = scroll_position_query.single_mut() {
49                    scroll_position.0 = (start.0 - drag.distance / ui_scale.0).max(Vec2::ZERO);
50                }
51            },
52        )
53        .observe(
54            |_: On<Pointer<DragStart>>,
55             mut scroll_position_query: Query<
56                (&ComputedNode, &mut ScrollStart),
57                With<ScrollableNode>,
58            >| {
59                if let Ok((computed_node, mut start)) = scroll_position_query.single_mut() {
60                    start.0 = computed_node.scroll_position * computed_node.inverse_scale_factor;
61                }
62            },
63        )
64        .with_children(|commands| {
65            commands
66                .spawn((
67                    Node {
68                        display: Display::Grid,
69                        grid_template_rows: RepeatedGridTrack::px(w as i32, 100.),
70                        grid_template_columns: RepeatedGridTrack::px(h as i32, 100.),
71                        ..default()
72                    },
73                    Pickable {
74                        is_hoverable: false,
75                        should_block_lower: true,
76                    }
77                ))
78                .with_children(|commands| {
79                    for y in 0..h {
80                        for x in 0..w {
81                            let tile_color = if (x + y) % 2 == 1 {
82                                let hue = ((x as f32 / w as f32) * 270.0)
83                                    + ((y as f32 / h as f32) * 90.0);
84                                Color::hsl(hue, 1., 0.5)
85                            } else {
86                                Color::BLACK
87                            };
88                            commands.spawn((
89                                Node {
90                                    grid_row: GridPlacement::start(y + 1),
91                                    grid_column: GridPlacement::start(x + 1),
92                                    ..default()
93                                },
94                                Pickable {
95                                    should_block_lower: false,
96                                    is_hoverable: true,
97                                },
98                                TileColor(tile_color),
99                                BackgroundColor(tile_color),
100                            ))
101                            .observe(|over: On<Pointer<Over>>, mut query: Query<&mut BackgroundColor>,| {
102                                if let Ok(mut background_color) = query.get_mut(over.entity) {
103                                    background_color.0 = RED.into();
104                                }
105                            })
106                            .observe(|out: On<Pointer<Out>>, mut query: Query<(&mut BackgroundColor, &TileColor)>| {
107                                if let Ok((mut background_color, tile_color)) = query.get_mut(out.entity) {
108                                    background_color.0 = tile_color.0;
109                                }
110                            });
111                        }
112                    }
113                });
114        });
115}
```

examples/testbed/ui.rs ([lines 1311-1314](../../src/testbed_ui/ui.rs.html#1311-1314))

```rust
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

examples/ui/text/multiple\_text\_inputs.rs ([line 60](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#60))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1846)

#### pub fn [percent](#method.percent)<T>(repetition: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")\>, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with a percentage size

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1855)

#### pub fn [auto](#method.auto)<T>(repetition: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with automatic size

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/ui/scroll\_and\_overflow/scroll.rs ([line 351](../../src/scroll/scroll.rs.html#351))

```rust
326fn bidirectional_scrolling_list_with_sticky(font_handle: Handle<Font>) -> impl Bundle {
327    (
328        Node {
329            flex_direction: FlexDirection::Column,
330            justify_content: JustifyContent::Center,
331            align_items: AlignItems::Center,
332            width: px(200),
333            ..default()
334        },
335        children![
336            (
337                Text::new("Bidirectionally Scrolling List With Sticky Nodes"),
338                TextFont {
339                    font: font_handle.clone().into(),
340                    font_size: FONT_SIZE,
341                    ..default()
342                },
343                Label,
344            ),
345            (
346                Node {
347                    display: Display::Grid,
348                    align_self: AlignSelf::Stretch,
349                    height: percent(50),
350                    overflow: Overflow::scroll(), // n.b.
351                    grid_template_columns: RepeatedGridTrack::auto(30),
352                    ..default()
353                },
354                Children::spawn(SpawnIter(
355                    (0..30)
356                        .flat_map(|y| (0..30).map(move |x| (y, x)))
357                        .map(move |(y, x)| {
358                            let value = font_handle.clone();
359                            // Simple sticky nodes at top and left sides of UI node
360                            // can be achieved by combining such effects as
361                            // IgnoreScroll, ZIndex, BackgroundColor for child UI nodes.
362                            let ignore_scroll = BVec2 {
363                                x: x == 0,
364                                y: y == 0,
365                            };
366                            let (z_index, background_color, role) = match (x == 0, y == 0) {
367                                (true, true) => (2, RED, Role::RowHeader),
368                                (true, false) => (1, BLUE, Role::RowHeader),
369                                (false, true) => (1, BLUE, Role::ColumnHeader),
370                                (false, false) => (0, BLACK, Role::Cell),
371                            };
372                            (
373                                Text(format!("|{},{}|", y, x)),
374                                TextFont::from(value.clone()),
375                                TextLayout {
376                                    linebreak: LineBreak::NoWrap,
377                                    ..default()
378                                },
379                                Label,
380                                AccessibilityNode(Accessible::new(role)),
381                                IgnoreScroll(ignore_scroll),
382                                ZIndex(z_index),
383                                BackgroundColor(Color::Srgba(background_color)),
384                            )
385                        })
386                ))
387            )
388        ],
389    )
390}
```

Hide additional examples

examples/ui/scroll\_and\_overflow/scrollbars.rs ([line 58](../../src/scrollbars/scrollbars.rs.html#58))

```rust
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

examples/ui/text/multiple\_text\_inputs.rs ([line 61](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#61))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1866)

#### pub fn [fr](#method.fr)<T>(repetition: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html), value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with an `fr` size. Note that this will give the track a content-based minimum size. Usually you are best off using `GridTrack::flex` instead which uses a zero minimum size.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/testbed/ui.rs ([line 1128](../../src/testbed_ui/ui.rs.html#1128))

```rust
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
```

Hide additional examples

examples/ui/text/generic\_font\_families.rs ([line 40](../../src/generic_font_families/generic_font_families.rs.html#40))

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

examples/ui/layout/anchor\_layout.rs ([line 107](../../src/anchor_layout/anchor_layout.rs.html#107))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1875)

#### pub fn [flex](#method.flex)<T>(repetition: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html), value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with a `minmax(0, Nfr)` size.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/stress\_tests/many\_buttons.rs ([line 236](../../src/many_buttons/many_buttons.rs.html#236))

```rust
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
```

Hide additional examples

examples/stress\_tests/many\_gradients.rs ([line 90](../../src/many_gradients/many_gradients.rs.html#90))

```rust
79fn setup(mut commands: Commands, args: Res<Args>) {
80    commands.spawn(Camera2d);
81
82    let rows_to_spawn = args.gradient_count.div_ceil(COLS);
83
84    // Create a grid of gradients
85    commands
86        .spawn(Node {
87            width: percent(100),
88            height: percent(100),
89            display: Display::Grid,
90            grid_template_columns: RepeatedGridTrack::flex(COLS as u16, 1.0),
91            grid_template_rows: RepeatedGridTrack::flex(rows_to_spawn as u16, 1.0),
92            ..default()
93        })
94        .with_children(|parent| {
95            for i in 0..args.gradient_count {
96                let angle = (i as f32 * 10.0) % 360.0;
97
98                let mut gradient = LinearGradient::new(
99                    angle,
100                    vec![
101                        ColorStop::new(RED, percent(0)),
102                        ColorStop::new(BLUE, percent(100)),
103                        ColorStop::new(GREEN, percent(20)),
104                        ColorStop::new(YELLOW, percent(40)),
105                        ColorStop::new(ORANGE, percent(60)),
106                        ColorStop::new(LIME, percent(80)),
107                        ColorStop::new(DARK_CYAN, percent(90)),
108                    ],
109                );
110
111                gradient.color_space = if args.srgb {
112                    InterpolationColorSpace::Srgba
113                } else if args.hsl {
114                    InterpolationColorSpace::Hsla
115                } else {
116                    InterpolationColorSpace::Oklaba
117                };
118
119                parent.spawn((
120                    Node {
121                        width: percent(100),
122                        height: percent(100),
123                        ..default()
124                    },
125                    BackgroundGradient(vec![Gradient::Linear(gradient)]),
126                    GradientNode { index: i },
127                ));
128            }
129        });
130}
```

examples/testbed/ui.rs ([line 719](../../src/testbed_ui/ui.rs.html#719))

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

examples/ui/scroll\_and\_overflow/scrollbars.rs ([line 58](../../src/scrollbars/scrollbars.rs.html#58))

```rust
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

examples/ui/layout/grid.rs ([line 79](../../src/grid/grid.rs.html#79))

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1884)

#### pub fn [min\_content](#method.min_content)<T>(repetition: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with min-content size

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1893)

#### pub fn [max\_content](#method.max_content)<T>(repetition: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with max-content size

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1902)

#### pub fn [fit\_content\_px](#method.fit_content_px)<T>(repetition: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html), limit: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of `fit-content()` grid tracks with fixed pixel limit

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1911)

#### pub fn [fit\_content\_percent](#method.fit_content_percent)<T>(repetition: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html), limit: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of `fit-content()` grid tracks with percentage limit

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1920-1924)

#### pub fn [minmax](#method.minmax)<T>( repetition: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")\>, min: [MinTrackSizingFunction](enum.MinTrackSizingFunction.html "enum bevy::prelude::MinTrackSizingFunction"), max: [MaxTrackSizingFunction](enum.MaxTrackSizingFunction.html "enum bevy::prelude::MaxTrackSizingFunction"), ) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of `minmax()` grid track

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1933)

#### pub fn [vmin](#method.vmin)<T>(repetition: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")\>, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with the percentage size of the viewport’s smaller dimension

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1942)

#### pub fn [vmax](#method.vmax)<T>(repetition: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")\>, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with the percentage size of the viewport’s larger dimension

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1951)

#### pub fn [vh](#method.vh)<T>(repetition: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")\>, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with the percentage size of the viewport’s height dimension

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1960)

#### pub fn [vw](#method.vw)<T>(repetition: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")\>, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repeating set of grid tracks with the percentage size of the viewport’s width dimension

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1969-1972)

#### pub fn [repeat\_many](#method.repeat_many)<T>( repetition: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")\>, tracks: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec](struct.Vec.html "struct bevy::prelude::Vec")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\>>, ) -> T

where T: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>,

Create a repetition of a set of tracks

## Trait Implementations

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1981)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1982)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1827)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1827)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1990)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")\> for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1991)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(track: [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")) -> [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2014)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\> for [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2015)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(track: [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")) -> [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)( arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [from\_reflect](trait.FromReflect.html#tymethod.from_reflect)( reflect: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [get\_represented\_type\_info](trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [try\_apply](trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [reflect\_kind](trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [reflect\_ref](trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [reflect\_mut](trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [reflect\_owned](trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [try\_into\_reflect](trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](struct.Box.html "struct bevy::prelude::Box")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [try\_as\_reflect](trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [try\_as\_reflect\_mut](trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [into\_partial\_reflect](trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [as\_partial\_reflect](trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [as\_partial\_reflect\_mut](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1824)

#### fn [reflect\_partial\_eq](trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [reflect\_partial\_cmp](trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1824)

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

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [into\_any](trait.Reflect.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [as\_any](trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [as\_any\_mut](trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [into\_reflect](trait.Reflect.html#tymethod.into_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [as\_reflect](trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [as\_reflect\_mut](trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [set](trait.Reflect.html#tymethod.set)(&mut self, value: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1827)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1827)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [field](trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [field\_mut](trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [field\_at](trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [field\_at\_mut](trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [name\_at](trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [index\_of\_name](trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [field\_len](trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [iter\_fields](trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [to\_dynamic\_struct](trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [type\_path](trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [short\_type\_path](trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [type\_ident](trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [crate\_name](trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [module\_path](trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

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